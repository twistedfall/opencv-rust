#![allow(unused_parens)]
//! # Feature Detection and Description
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CUDA_DescriptorMatcher, super::CUDA_Feature2DAsync, super::CUDA_FastFeatureDetector, super::CUDA_ORB };
}

/// Abstract base class for matching keypoint descriptors.
/// 
/// It has two groups of match methods: for matching descriptors of an image with another image or with
/// an image set.
pub trait CUDA_DescriptorMatcher: core::AlgorithmTrait {
	fn as_raw_CUDA_DescriptorMatcher(&self) -> *const c_void;
	fn as_raw_mut_CUDA_DescriptorMatcher(&mut self) -> *mut c_void;

	/// Returns true if the descriptor matcher supports masking permissible matches.
	fn is_mask_supported(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DescriptorMatcher_isMaskSupported_const(self.as_raw_CUDA_DescriptorMatcher()) }.into_result()
	}
	
	/// Adds descriptors to train a descriptor collection.
	/// 
	/// If the collection is not empty, the new descriptors are added to existing train descriptors.
	/// 
	/// ## Parameters
	/// * descriptors: Descriptors to add. Each descriptors[i] is a set of descriptors from the same
	/// train image.
	fn add(&mut self, descriptors: &core::Vector::<core::GpuMat>) -> Result<()> {
		unsafe { sys::cv_cuda_DescriptorMatcher_add_const_vector_GpuMat_R(self.as_raw_mut_CUDA_DescriptorMatcher(), descriptors.as_raw_VectorOfGpuMat()) }.into_result()
	}
	
	/// Returns a constant link to the train descriptor collection.
	fn get_train_descriptors(&self) -> Result<core::Vector::<core::GpuMat>> {
		unsafe { sys::cv_cuda_DescriptorMatcher_getTrainDescriptors_const(self.as_raw_CUDA_DescriptorMatcher()) }.into_result().map(|r| unsafe { core::Vector::<core::GpuMat>::opencv_from_extern(r) } )
	}
	
	/// Clears the train descriptor collection.
	fn clear(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_DescriptorMatcher_clear(self.as_raw_mut_CUDA_DescriptorMatcher()) }.into_result()
	}
	
	/// Returns true if there are no train descriptors in the collection.
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DescriptorMatcher_empty_const(self.as_raw_CUDA_DescriptorMatcher()) }.into_result()
	}
	
	/// Trains a descriptor matcher.
	/// 
	/// Trains a descriptor matcher (for example, the flann index). In all methods to match, the method
	/// train() is run every time before matching.
	fn train(&mut self) -> Result<()> {
		unsafe { sys::cv_cuda_DescriptorMatcher_train(self.as_raw_mut_CUDA_DescriptorMatcher()) }.into_result()
	}
	
	/// Finds the best match for each descriptor from a query set (blocking version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches. If a query descriptor is masked out in mask , no match is added for this
	/// descriptor. So, matches size may be smaller than the query descriptors count.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// 
	/// In the first variant of this method, the train descriptors are passed as an input argument. In the
	/// second variant of the method, train descriptors collection that was set by DescriptorMatcher::add is
	/// used. Optional mask (or masks) can be passed to specify which query and training descriptors can be
	/// matched. Namely, queryDescriptors[i] can be matched with trainDescriptors[j] only if
	/// mask.at\<uchar\>(i,j) is non-zero.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	fn match_(&mut self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::DMatch>, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vector_DMatch_R_const__InputArrayR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw__InputArray()) }.into_result()
	}
	
	/// Finds the best match for each descriptor from a query set (blocking version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches. If a query descriptor is masked out in mask , no match is added for this
	/// descriptor. So, matches size may be smaller than the query descriptors count.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// 
	/// In the first variant of this method, the train descriptors are passed as an input argument. In the
	/// second variant of the method, train descriptors collection that was set by DescriptorMatcher::add is
	/// used. Optional mask (or masks) can be passed to specify which query and training descriptors can be
	/// matched. Namely, queryDescriptors[i] can be matched with trainDescriptors[j] only if
	/// mask.at\<uchar\>(i,j) is non-zero.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * masks: std::vector<GpuMat>()
	fn match__1(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::DMatch>, masks: &core::Vector::<core::GpuMat>) -> Result<()> {
		input_array_arg!(query_descriptors);
		unsafe { sys::cv_cuda_DescriptorMatcher_match_const__InputArrayR_vector_DMatch_R_const_vector_GpuMat_R(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw_VectorOfGpuMat()) }.into_result()
	}
	
	/// Finds the best match for each descriptor from a query set (asynchronous version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches array stored in GPU memory. Internal representation is not defined.
	/// Use DescriptorMatcher::matchConvert method to retrieve results in standard representation.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * stream: CUDA stream.
	/// 
	/// In the first variant of this method, the train descriptors are passed as an input argument. In the
	/// second variant of the method, train descriptors collection that was set by DescriptorMatcher::add is
	/// used. Optional mask (or masks) can be passed to specify which query and training descriptors can be
	/// matched. Namely, queryDescriptors[i] can be matched with trainDescriptors[j] only if
	/// mask.at\<uchar\>(i,j) is non-zero.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	fn match_async(&mut self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		output_array_arg!(matches);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Finds the best match for each descriptor from a query set (asynchronous version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches array stored in GPU memory. Internal representation is not defined.
	/// Use DescriptorMatcher::matchConvert method to retrieve results in standard representation.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * stream: CUDA stream.
	/// 
	/// In the first variant of this method, the train descriptors are passed as an input argument. In the
	/// second variant of the method, train descriptors collection that was set by DescriptorMatcher::add is
	/// used. Optional mask (or masks) can be passed to specify which query and training descriptors can be
	/// matched. Namely, queryDescriptors[i] can be matched with trainDescriptors[j] only if
	/// mask.at\<uchar\>(i,j) is non-zero.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * masks: std::vector<GpuMat>()
	/// * stream: Stream::Null()
	fn match_async_1(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray, masks: &core::Vector::<core::GpuMat>, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(query_descriptors);
		output_array_arg!(matches);
		unsafe { sys::cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR_const_vector_GpuMat_R_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), masks.as_raw_VectorOfGpuMat(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Converts matches array from internal representation to standard matches vector.
	/// 
	/// The method is supposed to be used with DescriptorMatcher::matchAsync to get final result.
	/// Call this method only after DescriptorMatcher::matchAsync is completed (ie. after synchronization).
	/// 
	/// ## Parameters
	/// * gpu_matches: Matches, returned from DescriptorMatcher::matchAsync.
	/// * matches: Vector of DMatch objects.
	fn match_convert(&mut self, gpu_matches: &dyn core::ToInputArray, matches: &mut core::Vector::<core::DMatch>) -> Result<()> {
		input_array_arg!(gpu_matches);
		unsafe { sys::cv_cuda_DescriptorMatcher_matchConvert_const__InputArrayR_vector_DMatch_R(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch()) }.into_result()
	}
	
	/// Finds the k best matches for each descriptor from a query set (blocking version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches. Each matches[i] is k or less matches for the same query descriptor.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	/// less than k possible matches in total.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// These extended variants of DescriptorMatcher::match methods find several best matches for each query
	/// descriptor. The matches are returned in the distance increasing order. See DescriptorMatcher::match
	/// for the details about query and train descriptors.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * compact_result: false
	fn knn_match(&mut self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, k: i32, mask: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_int_const__InputArrayR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw__InputArray(), compact_result) }.into_result()
	}
	
	/// Finds the k best matches for each descriptor from a query set (blocking version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches. Each matches[i] is k or less matches for the same query descriptor.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	/// less than k possible matches in total.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// These extended variants of DescriptorMatcher::match methods find several best matches for each query
	/// descriptor. The matches are returned in the distance increasing order. See DescriptorMatcher::match
	/// for the details about query and train descriptors.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * masks: std::vector<GpuMat>()
	/// * compact_result: false
	fn knn_match_1(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, k: i32, masks: &core::Vector::<core::GpuMat>, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		unsafe { sys::cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vector_vector_DMatch__R_int_const_vector_GpuMat_R_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw_VectorOfGpuMat(), compact_result) }.into_result()
	}
	
	/// Finds the k best matches for each descriptor from a query set (asynchronous version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches array stored in GPU memory. Internal representation is not defined.
	/// Use DescriptorMatcher::knnMatchConvert method to retrieve results in standard representation.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	/// less than k possible matches in total.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * stream: CUDA stream.
	/// 
	/// These extended variants of DescriptorMatcher::matchAsync methods find several best matches for each query
	/// descriptor. The matches are returned in the distance increasing order. See DescriptorMatcher::matchAsync
	/// for the details about query and train descriptors.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	fn knn_match_async(&mut self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray, k: i32, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		output_array_arg!(matches);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), k, mask.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Finds the k best matches for each descriptor from a query set (asynchronous version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches array stored in GPU memory. Internal representation is not defined.
	/// Use DescriptorMatcher::knnMatchConvert method to retrieve results in standard representation.
	/// * k: Count of best matches found per each query descriptor or less if a query descriptor has
	/// less than k possible matches in total.
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * stream: CUDA stream.
	/// 
	/// These extended variants of DescriptorMatcher::matchAsync methods find several best matches for each query
	/// descriptor. The matches are returned in the distance increasing order. See DescriptorMatcher::matchAsync
	/// for the details about query and train descriptors.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * masks: std::vector<GpuMat>()
	/// * stream: Stream::Null()
	fn knn_match_async_1(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray, k: i32, masks: &core::Vector::<core::GpuMat>, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(query_descriptors);
		output_array_arg!(matches);
		unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int_const_vector_GpuMat_R_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), k, masks.as_raw_VectorOfGpuMat(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Converts matches array from internal representation to standard matches vector.
	/// 
	/// The method is supposed to be used with DescriptorMatcher::knnMatchAsync to get final result.
	/// Call this method only after DescriptorMatcher::knnMatchAsync is completed (ie. after synchronization).
	/// 
	/// ## Parameters
	/// * gpu_matches: Matches, returned from DescriptorMatcher::knnMatchAsync.
	/// * matches: Vector of DMatch objects.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// ## C++ default parameters
	/// * compact_result: false
	fn knn_match_convert(&mut self, gpu_matches: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, compact_result: bool) -> Result<()> {
		input_array_arg!(gpu_matches);
		unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vector_vector_DMatch__R_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), compact_result) }.into_result()
	}
	
	/// For each query descriptor, finds the training descriptors not farther than the specified distance (blocking version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Found matches.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// For each query descriptor, the methods find such training descriptors that the distance between the
	/// query descriptor and the training descriptor is equal or smaller than maxDistance. Found matches are
	/// returned in the distance increasing order.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * compact_result: false
	fn radius_match(&mut self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, max_distance: f32, mask: &dyn core::ToInputArray, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vector_vector_DMatch__R_float_const__InputArrayR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw__InputArray(), compact_result) }.into_result()
	}
	
	/// For each query descriptor, finds the training descriptors not farther than the specified distance (blocking version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Found matches.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// For each query descriptor, the methods find such training descriptors that the distance between the
	/// query descriptor and the training descriptor is equal or smaller than maxDistance. Found matches are
	/// returned in the distance increasing order.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * masks: std::vector<GpuMat>()
	/// * compact_result: false
	fn radius_match_1(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, max_distance: f32, masks: &core::Vector::<core::GpuMat>, compact_result: bool) -> Result<()> {
		input_array_arg!(query_descriptors);
		unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vector_vector_DMatch__R_float_const_vector_GpuMat_R_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw_VectorOfGpuMat(), compact_result) }.into_result()
	}
	
	/// For each query descriptor, finds the training descriptors not farther than the specified distance (asynchronous version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches array stored in GPU memory. Internal representation is not defined.
	/// Use DescriptorMatcher::radiusMatchConvert method to retrieve results in standard representation.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * stream: CUDA stream.
	/// 
	/// For each query descriptor, the methods find such training descriptors that the distance between the
	/// query descriptor and the training descriptor is equal or smaller than maxDistance. Found matches are
	/// returned in the distance increasing order.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	fn radius_match_async(&mut self, query_descriptors: &dyn core::ToInputArray, train_descriptors: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray, max_distance: f32, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(query_descriptors);
		input_array_arg!(train_descriptors);
		output_array_arg!(matches);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), max_distance, mask.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// For each query descriptor, finds the training descriptors not farther than the specified distance (asynchronous version).
	/// 
	/// ## Parameters
	/// * queryDescriptors: Query set of descriptors.
	/// * trainDescriptors: Train set of descriptors. This set is not added to the train descriptors
	/// collection stored in the class object.
	/// * matches: Matches array stored in GPU memory. Internal representation is not defined.
	/// Use DescriptorMatcher::radiusMatchConvert method to retrieve results in standard representation.
	/// * maxDistance: Threshold for the distance between matched descriptors. Distance means here
	/// metric distance (e.g. Hamming distance), not the distance between coordinates (which is measured
	/// in Pixels)!
	/// * mask: Mask specifying permissible matches between an input query and train matrices of
	/// descriptors.
	/// * stream: CUDA stream.
	/// 
	/// For each query descriptor, the methods find such training descriptors that the distance between the
	/// query descriptor and the training descriptor is equal or smaller than maxDistance. Found matches are
	/// returned in the distance increasing order.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * masks: std::vector<GpuMat>()
	/// * stream: Stream::Null()
	fn radius_match_async_1(&mut self, query_descriptors: &dyn core::ToInputArray, matches: &mut dyn core::ToOutputArray, max_distance: f32, masks: &core::Vector::<core::GpuMat>, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(query_descriptors);
		output_array_arg!(matches);
		unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float_const_vector_GpuMat_R_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), max_distance, masks.as_raw_VectorOfGpuMat(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Converts matches array from internal representation to standard matches vector.
	/// 
	/// The method is supposed to be used with DescriptorMatcher::radiusMatchAsync to get final result.
	/// Call this method only after DescriptorMatcher::radiusMatchAsync is completed (ie. after synchronization).
	/// 
	/// ## Parameters
	/// * gpu_matches: Matches, returned from DescriptorMatcher::radiusMatchAsync.
	/// * matches: Vector of DMatch objects.
	/// * compactResult: Parameter used when the mask (or masks) is not empty. If compactResult is
	/// false, the matches vector has the same size as queryDescriptors rows. If compactResult is true,
	/// the matches vector does not contain matches for fully masked-out query descriptors.
	/// 
	/// ## C++ default parameters
	/// * compact_result: false
	fn radius_match_convert(&mut self, gpu_matches: &dyn core::ToInputArray, matches: &mut core::Vector::<core::Vector::<core::DMatch>>, compact_result: bool) -> Result<()> {
		input_array_arg!(gpu_matches);
		unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vector_vector_DMatch__R_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), compact_result) }.into_result()
	}
	
}

impl dyn CUDA_DescriptorMatcher + '_ {
	/// Brute-force descriptor matcher.
	/// 
	/// For each descriptor in the first set, this matcher finds the closest descriptor in the second set
	/// by trying each one. This descriptor matcher supports masking permissible matches of descriptor
	/// sets.
	/// 
	/// ## Parameters
	/// * normType: One of NORM_L1, NORM_L2, NORM_HAMMING. L1 and L2 norms are
	/// preferable choices for SIFT and SURF descriptors, NORM_HAMMING should be used with ORB, BRISK and
	/// BRIEF).
	/// 
	/// ## C++ default parameters
	/// * norm_type: cv::NORM_L2
	pub fn create_bf_matcher(norm_type: i32) -> Result<core::Ptr::<dyn crate::cudafeatures2d::CUDA_DescriptorMatcher>> {
		unsafe { sys::cv_cuda_DescriptorMatcher_createBFMatcher_int(norm_type) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudafeatures2d::CUDA_DescriptorMatcher>::opencv_from_extern(r) } )
	}
	
}
/// Wrapping class for feature detection using the FAST method.
pub trait CUDA_FastFeatureDetector: crate::cudafeatures2d::CUDA_Feature2DAsync {
	fn as_raw_CUDA_FastFeatureDetector(&self) -> *const c_void;
	fn as_raw_mut_CUDA_FastFeatureDetector(&mut self) -> *mut c_void;

	fn set_threshold(&mut self, threshold: i32) -> Result<()> {
		unsafe { sys::cv_cuda_FastFeatureDetector_setThreshold_int(self.as_raw_mut_CUDA_FastFeatureDetector(), threshold) }.into_result()
	}
	
	fn set_max_num_points(&mut self, max_npoints: i32) -> Result<()> {
		unsafe { sys::cv_cuda_FastFeatureDetector_setMaxNumPoints_int(self.as_raw_mut_CUDA_FastFeatureDetector(), max_npoints) }.into_result()
	}
	
	fn get_max_num_points(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_FastFeatureDetector_getMaxNumPoints_const(self.as_raw_CUDA_FastFeatureDetector()) }.into_result()
	}
	
}

impl dyn CUDA_FastFeatureDetector + '_ {
	pub const LOCATION_ROW: i32 = 0;
	pub const RESPONSE_ROW: i32 = 1;
	pub const ROWS_COUNT: i32 = 2;
	pub const FEATURE_SIZE: i32 = 7;
	/// ## C++ default parameters
	/// * threshold: 10
	/// * nonmax_suppression: true
	/// * typ: cv::FastFeatureDetector::TYPE_9_16
	/// * max_npoints: 5000
	pub fn create(threshold: i32, nonmax_suppression: bool, typ: i32, max_npoints: i32) -> Result<core::Ptr::<dyn crate::cudafeatures2d::CUDA_FastFeatureDetector>> {
		unsafe { sys::cv_cuda_FastFeatureDetector_create_int_bool_int_int(threshold, nonmax_suppression, typ, max_npoints) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudafeatures2d::CUDA_FastFeatureDetector>::opencv_from_extern(r) } )
	}
	
}
/// Abstract base class for CUDA asynchronous 2D image feature detectors and descriptor extractors.
pub trait CUDA_Feature2DAsync: crate::features2d::Feature2DTrait {
	fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void;
	fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void;

	/// Detects keypoints in an image.
	/// 
	/// ## Parameters
	/// * image: Image.
	/// * keypoints: The detected keypoints.
	/// * mask: Mask specifying where to look for keypoints (optional). It must be a 8-bit integer
	/// matrix with non-zero values in the region of interest.
	/// * stream: CUDA stream.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	fn detect_async(&mut self, image: &dyn core::ToInputArray, keypoints: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(keypoints);
		input_array_arg!(mask);
		unsafe { sys::cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), keypoints.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Computes the descriptors for a set of keypoints detected in an image.
	/// 
	/// ## Parameters
	/// * image: Image.
	/// * keypoints: Input collection of keypoints.
	/// * descriptors: Computed descriptors. Row j is the descriptor for j-th keypoint.
	/// * stream: CUDA stream.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn compute_async(&mut self, image: &dyn core::ToInputArray, keypoints: &mut dyn core::ToOutputArray, descriptors: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(keypoints);
		output_array_arg!(descriptors);
		unsafe { sys::cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), keypoints.as_raw__OutputArray(), descriptors.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Detects keypoints and computes the descriptors.
	/// 
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	/// * stream: Stream::Null()
	fn detect_and_compute_async(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, keypoints: &mut dyn core::ToOutputArray, descriptors: &mut dyn core::ToOutputArray, use_provided_keypoints: bool, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(keypoints);
		output_array_arg!(descriptors);
		unsafe { sys::cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw__OutputArray(), descriptors.as_raw__OutputArray(), use_provided_keypoints, stream.as_raw_mut_Stream()) }.into_result()
	}
	
	/// Converts keypoints array from internal representation to standard vector.
	fn convert(&mut self, gpu_keypoints: &dyn core::ToInputArray, keypoints: &mut core::Vector::<core::KeyPoint>) -> Result<()> {
		input_array_arg!(gpu_keypoints);
		unsafe { sys::cv_cuda_Feature2DAsync_convert_const__InputArrayR_vector_KeyPoint_R(self.as_raw_mut_CUDA_Feature2DAsync(), gpu_keypoints.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint()) }.into_result()
	}
	
}

/// Class implementing the ORB (*oriented BRIEF*) keypoint detector and descriptor extractor
/// ## See also
/// cv::ORB
pub trait CUDA_ORB: crate::cudafeatures2d::CUDA_Feature2DAsync {
	fn as_raw_CUDA_ORB(&self) -> *const c_void;
	fn as_raw_mut_CUDA_ORB(&mut self) -> *mut c_void;

	/// if true, image will be blurred before descriptors calculation
	fn set_blur_for_descriptor(&mut self, blur_for_descriptor: bool) -> Result<()> {
		unsafe { sys::cv_cuda_ORB_setBlurForDescriptor_bool(self.as_raw_mut_CUDA_ORB(), blur_for_descriptor) }.into_result()
	}
	
	fn get_blur_for_descriptor(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_ORB_getBlurForDescriptor_const(self.as_raw_CUDA_ORB()) }.into_result()
	}
	
	fn set_fast_threshold(&mut self, fast_threshold: i32) -> Result<()> {
		unsafe { sys::cv_cuda_ORB_setFastThreshold_int(self.as_raw_mut_CUDA_ORB(), fast_threshold) }.into_result()
	}
	
	fn get_fast_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_ORB_getFastThreshold_const(self.as_raw_CUDA_ORB()) }.into_result()
	}
	
}

impl dyn CUDA_ORB + '_ {
	pub const X_ROW: i32 = 0;
	pub const Y_ROW: i32 = 1;
	pub const RESPONSE_ROW: i32 = 2;
	pub const ANGLE_ROW: i32 = 3;
	pub const OCTAVE_ROW: i32 = 4;
	pub const SIZE_ROW: i32 = 5;
	pub const ROWS_COUNT: i32 = 6;
	/// ## C++ default parameters
	/// * nfeatures: 500
	/// * scale_factor: 1.2f
	/// * nlevels: 8
	/// * edge_threshold: 31
	/// * first_level: 0
	/// * wta_k: 2
	/// * score_type: cv::ORB::HARRIS_SCORE
	/// * patch_size: 31
	/// * fast_threshold: 20
	/// * blur_for_descriptor: false
	pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: i32, patch_size: i32, fast_threshold: i32, blur_for_descriptor: bool) -> Result<core::Ptr::<dyn crate::cudafeatures2d::CUDA_ORB>> {
		unsafe { sys::cv_cuda_ORB_create_int_float_int_int_int_int_int_int_int_bool(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold, blur_for_descriptor) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudafeatures2d::CUDA_ORB>::opencv_from_extern(r) } )
	}
	
}
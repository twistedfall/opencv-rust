pub mod cudafeatures2d {
	//! # Feature Detection and Description
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::CUDA_DescriptorMatcherTraitConst, super::CUDA_DescriptorMatcherTrait, super::CUDA_Feature2DAsyncTraitConst, super::CUDA_Feature2DAsyncTrait, super::CUDA_FastFeatureDetectorTraitConst, super::CUDA_FastFeatureDetectorTrait, super::CUDA_ORBTraitConst, super::CUDA_ORBTrait };
	}
	
	/// Constant methods for [crate::cudafeatures2d::CUDA_DescriptorMatcher]
	pub trait CUDA_DescriptorMatcherTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_DescriptorMatcher(&self) -> *const c_void;
	
		/// Returns true if the descriptor matcher supports masking permissible matches.
		#[inline]
		fn is_mask_supported(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_isMaskSupported_const(self.as_raw_CUDA_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns a constant link to the train descriptor collection.
		#[inline]
		fn get_train_descriptors(&self) -> Result<core::Vector<core::GpuMat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_getTrainDescriptors_const(self.as_raw_CUDA_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::GpuMat>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns true if there are no train descriptors in the collection.
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_empty_const(self.as_raw_CUDA_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudafeatures2d::CUDA_DescriptorMatcher]
	pub trait CUDA_DescriptorMatcherTrait: core::AlgorithmTrait + crate::cudafeatures2d::CUDA_DescriptorMatcherTraitConst {
		fn as_raw_mut_CUDA_DescriptorMatcher(&mut self) -> *mut c_void;
	
		/// Adds descriptors to train a descriptor collection.
		/// 
		/// If the collection is not empty, the new descriptors are added to existing train descriptors.
		/// 
		/// ## Parameters
		/// * descriptors: Descriptors to add. Each descriptors[i] is a set of descriptors from the same
		/// train image.
		#[inline]
		fn add(&mut self, descriptors: &core::Vector<core::GpuMat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_add_const_vectorLGpuMatGR(self.as_raw_mut_CUDA_DescriptorMatcher(), descriptors.as_raw_VectorOfGpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Clears the train descriptor collection.
		#[inline]
		fn clear(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_clear(self.as_raw_mut_CUDA_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Trains a descriptor matcher.
		/// 
		/// Trains a descriptor matcher (for example, the flann index). In all methods to match, the method
		/// train() is run every time before matching.
		#[inline]
		fn train(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_train(self.as_raw_mut_CUDA_DescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn match_(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>, mask: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vectorLDMatchGR_const__InputArrayR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [match_] function uses the following default values for its arguments:
		/// * mask: noArray()
		#[inline]
		fn match__def(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_match_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn match__1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>, masks: &core::Vector<core::GpuMat>) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR_const_vectorLGpuMatGR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw_VectorOfGpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [match_] function uses the following default values for its arguments:
		/// * masks: std::vector<GpuMat>()
		#[inline]
		fn match__def_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_match_const__InputArrayR_vectorLDMatchGR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn match_async(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			output_array_arg!(matches);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [match_async] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn match_async_def(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn match_async_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, masks: &core::Vector<core::GpuMat>, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(query_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR_const_vectorLGpuMatGR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), masks.as_raw_VectorOfGpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [match_async] function uses the following default values for its arguments:
		/// * masks: std::vector<GpuMat>()
		/// * stream: Stream::Null()
		#[inline]
		fn match_async_def_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(query_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_matchAsync_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Converts matches array from internal representation to standard matches vector.
		/// 
		/// The method is supposed to be used with DescriptorMatcher::matchAsync to get final result.
		/// Call this method only after DescriptorMatcher::matchAsync is completed (ie. after synchronization).
		/// 
		/// ## Parameters
		/// * gpu_matches: Matches, returned from DescriptorMatcher::matchAsync.
		/// * matches: Vector of DMatch objects.
		#[inline]
		fn match_convert(&mut self, gpu_matches: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
			input_array_arg!(gpu_matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_matchConvert_const__InputArrayR_vectorLDMatchGR(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn knn_match(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, mask: &impl core::ToInputArray, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int_const__InputArrayR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [knn_match] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * compact_result: false
		#[inline]
		fn knn_match_def(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_int(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn knn_match_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, masks: &core::Vector<core::GpuMat>, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int_const_vectorLGpuMatGR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw_VectorOfGpuMat(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [knn_match] function uses the following default values for its arguments:
		/// * masks: std::vector<GpuMat>()
		/// * compact_result: false
		#[inline]
		fn knn_match_def_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatch_const__InputArrayR_vectorLvectorLDMatchGGR_int(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn knn_match_async(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, k: i32, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			output_array_arg!(matches);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), k, mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [knn_match_async] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn knn_match_async_def(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, k: i32) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn knn_match_async_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, k: i32, masks: &core::Vector<core::GpuMat>, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(query_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int_const_vectorLGpuMatGR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), k, masks.as_raw_VectorOfGpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [knn_match_async] function uses the following default values for its arguments:
		/// * masks: std::vector<GpuMat>()
		/// * stream: Stream::Null()
		#[inline]
		fn knn_match_async_def_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, k: i32) -> Result<()> {
			input_array_arg!(query_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchAsync_const__InputArrayR_const__OutputArrayR_int(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn knn_match_convert(&mut self, gpu_matches: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, compact_result: bool) -> Result<()> {
			input_array_arg!(gpu_matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [knn_match_convert] function uses the following default values for its arguments:
		/// * compact_result: false
		#[inline]
		fn knn_match_convert_def(&mut self, gpu_matches: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>) -> Result<()> {
			input_array_arg!(gpu_matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_knnMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn radius_match(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, mask: &impl core::ToInputArray, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float_const__InputArrayR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw__InputArray(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [radius_match] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * compact_result: false
		#[inline]
		fn radius_match_def(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn radius_match_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, masks: &core::Vector<core::GpuMat>, compact_result: bool) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float_const_vectorLGpuMatGR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw_VectorOfGpuMat(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [radius_match] function uses the following default values for its arguments:
		/// * masks: std::vector<GpuMat>()
		/// * compact_result: false
		#[inline]
		fn radius_match_def_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatch_const__InputArrayR_vectorLvectorLDMatchGGR_float(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn radius_match_async(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, max_distance: f32, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			output_array_arg!(matches);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), max_distance, mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [radius_match_async] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn radius_match_async_def(&mut self, query_descriptors: &impl core::ToInputArray, train_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			input_array_arg!(train_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), train_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn radius_match_async_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, max_distance: f32, masks: &core::Vector<core::GpuMat>, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(query_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float_const_vectorLGpuMatGR_StreamR(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), max_distance, masks.as_raw_VectorOfGpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [radius_match_async] function uses the following default values for its arguments:
		/// * masks: std::vector<GpuMat>()
		/// * stream: Stream::Null()
		#[inline]
		fn radius_match_async_def_1(&mut self, query_descriptors: &impl core::ToInputArray, matches: &mut impl core::ToOutputArray, max_distance: f32) -> Result<()> {
			input_array_arg!(query_descriptors);
			output_array_arg!(matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchAsync_const__InputArrayR_const__OutputArrayR_float(self.as_raw_mut_CUDA_DescriptorMatcher(), query_descriptors.as_raw__InputArray(), matches.as_raw__OutputArray(), max_distance, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn radius_match_convert(&mut self, gpu_matches: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>, compact_result: bool) -> Result<()> {
			input_array_arg!(gpu_matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR_bool(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), compact_result, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [radius_match_convert] function uses the following default values for its arguments:
		/// * compact_result: false
		#[inline]
		fn radius_match_convert_def(&mut self, gpu_matches: &impl core::ToInputArray, matches: &mut core::Vector<core::Vector<core::DMatch>>) -> Result<()> {
			input_array_arg!(gpu_matches);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_radiusMatchConvert_const__InputArrayR_vectorLvectorLDMatchGGR(self.as_raw_mut_CUDA_DescriptorMatcher(), gpu_matches.as_raw__InputArray(), matches.as_raw_mut_VectorOfVectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for matching keypoint descriptors.
	/// 
	/// It has two groups of match methods: for matching descriptors of an image with another image or with
	/// an image set.
	pub struct CUDA_DescriptorMatcher {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_DescriptorMatcher }
	
	impl Drop for CUDA_DescriptorMatcher {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_DescriptorMatcher_delete(self.as_raw_mut_CUDA_DescriptorMatcher()) };
		}
	}
	
	unsafe impl Send for CUDA_DescriptorMatcher {}
	
	impl core::AlgorithmTraitConst for CUDA_DescriptorMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_DescriptorMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_DescriptorMatcherTraitConst for CUDA_DescriptorMatcher {
		#[inline] fn as_raw_CUDA_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_DescriptorMatcherTrait for CUDA_DescriptorMatcher {
		#[inline] fn as_raw_mut_CUDA_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_DescriptorMatcher {
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
		#[inline]
		pub fn create_bf_matcher(norm_type: i32) -> Result<core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_createBFMatcher_int(norm_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudafeatures2d::CUDA_DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
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
		/// ## Note
		/// This alternative version of [create_bf_matcher] function uses the following default values for its arguments:
		/// * norm_type: cv::NORM_L2
		#[inline]
		pub fn create_bf_matcher_def() -> Result<core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DescriptorMatcher_createBFMatcher(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudafeatures2d::CUDA_DescriptorMatcher>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_DescriptorMatcher, core::Algorithm, cv_cuda_DescriptorMatcher_to_Algorithm }
	
	impl std::fmt::Debug for CUDA_DescriptorMatcher {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_DescriptorMatcher")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudafeatures2d::CUDA_FastFeatureDetector]
	pub trait CUDA_FastFeatureDetectorTraitConst: crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst {
		fn as_raw_CUDA_FastFeatureDetector(&self) -> *const c_void;
	
		#[inline]
		fn get_max_num_points(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FastFeatureDetector_getMaxNumPoints_const(self.as_raw_CUDA_FastFeatureDetector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudafeatures2d::CUDA_FastFeatureDetector]
	pub trait CUDA_FastFeatureDetectorTrait: crate::cudafeatures2d::CUDA_FastFeatureDetectorTraitConst + crate::cudafeatures2d::CUDA_Feature2DAsyncTrait {
		fn as_raw_mut_CUDA_FastFeatureDetector(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_threshold(&mut self, threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FastFeatureDetector_setThreshold_int(self.as_raw_mut_CUDA_FastFeatureDetector(), threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_num_points(&mut self, max_npoints: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FastFeatureDetector_setMaxNumPoints_int(self.as_raw_mut_CUDA_FastFeatureDetector(), max_npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Wrapping class for feature detection using the FAST method.
	pub struct CUDA_FastFeatureDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_FastFeatureDetector }
	
	impl Drop for CUDA_FastFeatureDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_FastFeatureDetector_delete(self.as_raw_mut_CUDA_FastFeatureDetector()) };
		}
	}
	
	unsafe impl Send for CUDA_FastFeatureDetector {}
	
	impl core::AlgorithmTraitConst for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTrait for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_FastFeatureDetectorTraitConst for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_CUDA_FastFeatureDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_FastFeatureDetectorTrait for CUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_CUDA_FastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_FastFeatureDetector {
		pub const LOCATION_ROW: i32 = 0;
		pub const RESPONSE_ROW: i32 = 1;
		pub const ROWS_COUNT: i32 = 2;
		pub const FEATURE_SIZE: i32 = 7;
		/// ## C++ default parameters
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: cv::FastFeatureDetector::TYPE_9_16
		/// * max_npoints: 5000
		#[inline]
		pub fn create(threshold: i32, nonmax_suppression: bool, typ: i32, max_npoints: i32) -> Result<core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FastFeatureDetector_create_int_bool_int_int(threshold, nonmax_suppression, typ, max_npoints, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudafeatures2d::CUDA_FastFeatureDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * threshold: 10
		/// * nonmax_suppression: true
		/// * typ: cv::FastFeatureDetector::TYPE_9_16
		/// * max_npoints: 5000
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FastFeatureDetector_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudafeatures2d::CUDA_FastFeatureDetector>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_FastFeatureDetector, core::Algorithm, cv_cuda_FastFeatureDetector_to_Algorithm }
	
	boxed_cast_base! { CUDA_FastFeatureDetector, crate::features2d::Feature2D, cv_cuda_FastFeatureDetector_to_Feature2D }
	
	boxed_cast_base! { CUDA_FastFeatureDetector, crate::cudafeatures2d::CUDA_Feature2DAsync, cv_cuda_FastFeatureDetector_to_CUDA_Feature2DAsync }
	
	impl std::fmt::Debug for CUDA_FastFeatureDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_FastFeatureDetector")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudafeatures2d::CUDA_Feature2DAsync]
	pub trait CUDA_Feature2DAsyncTraitConst: crate::features2d::Feature2DTraitConst {
		fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudafeatures2d::CUDA_Feature2DAsync]
	pub trait CUDA_Feature2DAsyncTrait: crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst + crate::features2d::Feature2DTrait {
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
		#[inline]
		fn detect_async(&mut self, image: &impl core::ToInputArray, keypoints: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(keypoints);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), keypoints.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects keypoints in an image.
		/// 
		/// ## Parameters
		/// * image: Image.
		/// * keypoints: The detected keypoints.
		/// * mask: Mask specifying where to look for keypoints (optional). It must be a 8-bit integer
		/// matrix with non-zero values in the region of interest.
		/// * stream: CUDA stream.
		/// 
		/// ## Note
		/// This alternative version of [detect_async] function uses the following default values for its arguments:
		/// * mask: noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn detect_async_def(&mut self, image: &impl core::ToInputArray, keypoints: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(keypoints);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Feature2DAsync_detectAsync_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), keypoints.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn compute_async(&mut self, image: &impl core::ToInputArray, keypoints: &mut impl core::ToOutputArray, descriptors: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(keypoints);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), keypoints.as_raw__OutputArray(), descriptors.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Computes the descriptors for a set of keypoints detected in an image.
		/// 
		/// ## Parameters
		/// * image: Image.
		/// * keypoints: Input collection of keypoints.
		/// * descriptors: Computed descriptors. Row j is the descriptor for j-th keypoint.
		/// * stream: CUDA stream.
		/// 
		/// ## Note
		/// This alternative version of [compute_async] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn compute_async_def(&mut self, image: &impl core::ToInputArray, keypoints: &mut impl core::ToOutputArray, descriptors: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(keypoints);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Feature2DAsync_computeAsync_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), keypoints.as_raw__OutputArray(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects keypoints and computes the descriptors.
		/// 
		/// ## C++ default parameters
		/// * use_provided_keypoints: false
		/// * stream: Stream::Null()
		#[inline]
		fn detect_and_compute_async(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, keypoints: &mut impl core::ToOutputArray, descriptors: &mut impl core::ToOutputArray, use_provided_keypoints: bool, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(mask);
			output_array_arg!(keypoints);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw__OutputArray(), descriptors.as_raw__OutputArray(), use_provided_keypoints, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detects keypoints and computes the descriptors.
		/// 
		/// ## Note
		/// This alternative version of [detect_and_compute_async] function uses the following default values for its arguments:
		/// * use_provided_keypoints: false
		/// * stream: Stream::Null()
		#[inline]
		fn detect_and_compute_async_def(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, keypoints: &mut impl core::ToOutputArray, descriptors: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(mask);
			output_array_arg!(keypoints);
			output_array_arg!(descriptors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Feature2DAsync_detectAndComputeAsync_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_Feature2DAsync(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw__OutputArray(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Converts keypoints array from internal representation to standard vector.
		#[inline]
		fn convert(&mut self, gpu_keypoints: &impl core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
			input_array_arg!(gpu_keypoints);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Feature2DAsync_convert_const__InputArrayR_vectorLKeyPointGR(self.as_raw_mut_CUDA_Feature2DAsync(), gpu_keypoints.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for CUDA asynchronous 2D image feature detectors and descriptor extractors.
	pub struct CUDA_Feature2DAsync {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_Feature2DAsync }
	
	impl Drop for CUDA_Feature2DAsync {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_Feature2DAsync_delete(self.as_raw_mut_CUDA_Feature2DAsync()) };
		}
	}
	
	unsafe impl Send for CUDA_Feature2DAsync {}
	
	impl core::AlgorithmTraitConst for CUDA_Feature2DAsync {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_Feature2DAsync {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for CUDA_Feature2DAsync {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for CUDA_Feature2DAsync {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst for CUDA_Feature2DAsync {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTrait for CUDA_Feature2DAsync {
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_Feature2DAsync {
	}
	
	boxed_cast_descendant! { CUDA_Feature2DAsync, crate::cudafeatures2d::CUDA_FastFeatureDetector, cv_cuda_Feature2DAsync_to_CUDA_FastFeatureDetector }
	
	boxed_cast_descendant! { CUDA_Feature2DAsync, crate::cudafeatures2d::CUDA_ORB, cv_cuda_Feature2DAsync_to_CUDA_ORB }
	
	boxed_cast_base! { CUDA_Feature2DAsync, core::Algorithm, cv_cuda_Feature2DAsync_to_Algorithm }
	
	boxed_cast_base! { CUDA_Feature2DAsync, crate::features2d::Feature2D, cv_cuda_Feature2DAsync_to_Feature2D }
	
	impl std::fmt::Debug for CUDA_Feature2DAsync {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_Feature2DAsync")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudafeatures2d::CUDA_ORB]
	pub trait CUDA_ORBTraitConst: crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst {
		fn as_raw_CUDA_ORB(&self) -> *const c_void;
	
		#[inline]
		fn get_max_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getMaxFeatures_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_scale_factor(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getScaleFactor_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_n_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getNLevels_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_edge_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getEdgeThreshold_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_first_level(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getFirstLevel_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_wta_k(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getWTA_K_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_score_type(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getScoreType_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_patch_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getPatchSize_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_fast_threshold(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getFastThreshold_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_blur_for_descriptor(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_getBlurForDescriptor_const(self.as_raw_CUDA_ORB(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudafeatures2d::CUDA_ORB]
	pub trait CUDA_ORBTrait: crate::cudafeatures2d::CUDA_Feature2DAsyncTrait + crate::cudafeatures2d::CUDA_ORBTraitConst {
		fn as_raw_mut_CUDA_ORB(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_max_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setMaxFeatures_int(self.as_raw_mut_CUDA_ORB(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setScaleFactor_double(self.as_raw_mut_CUDA_ORB(), scale_factor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_n_levels(&mut self, nlevels: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setNLevels_int(self.as_raw_mut_CUDA_ORB(), nlevels, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_edge_threshold(&mut self, edge_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setEdgeThreshold_int(self.as_raw_mut_CUDA_ORB(), edge_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_first_level(&mut self, first_level: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setFirstLevel_int(self.as_raw_mut_CUDA_ORB(), first_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_wta_k(&mut self, wta_k: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setWTA_K_int(self.as_raw_mut_CUDA_ORB(), wta_k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_score_type(&mut self, score_type: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setScoreType_int(self.as_raw_mut_CUDA_ORB(), score_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_patch_size(&mut self, patch_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setPatchSize_int(self.as_raw_mut_CUDA_ORB(), patch_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_fast_threshold(&mut self, fast_threshold: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setFastThreshold_int(self.as_raw_mut_CUDA_ORB(), fast_threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// if true, image will be blurred before descriptors calculation
		#[inline]
		fn set_blur_for_descriptor(&mut self, blur_for_descriptor: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_setBlurForDescriptor_bool(self.as_raw_mut_CUDA_ORB(), blur_for_descriptor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class implementing the ORB (*oriented BRIEF*) keypoint detector and descriptor extractor
	/// ## See also
	/// cv::ORB
	pub struct CUDA_ORB {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_ORB }
	
	impl Drop for CUDA_ORB {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_ORB_delete(self.as_raw_mut_CUDA_ORB()) };
		}
	}
	
	unsafe impl Send for CUDA_ORB {}
	
	impl core::AlgorithmTraitConst for CUDA_ORB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_ORB {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for CUDA_ORB {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for CUDA_ORB {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst for CUDA_ORB {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTrait for CUDA_ORB {
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_ORBTraitConst for CUDA_ORB {
		#[inline] fn as_raw_CUDA_ORB(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_ORBTrait for CUDA_ORB {
		#[inline] fn as_raw_mut_CUDA_ORB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_ORB {
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
		#[inline]
		pub fn create(nfeatures: i32, scale_factor: f32, nlevels: i32, edge_threshold: i32, first_level: i32, wta_k: i32, score_type: i32, patch_size: i32, fast_threshold: i32, blur_for_descriptor: bool) -> Result<core::Ptr<crate::cudafeatures2d::CUDA_ORB>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_create_int_float_int_int_int_int_int_int_int_bool(nfeatures, scale_factor, nlevels, edge_threshold, first_level, wta_k, score_type, patch_size, fast_threshold, blur_for_descriptor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudafeatures2d::CUDA_ORB>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
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
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudafeatures2d::CUDA_ORB>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ORB_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudafeatures2d::CUDA_ORB>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_ORB, core::Algorithm, cv_cuda_ORB_to_Algorithm }
	
	boxed_cast_base! { CUDA_ORB, crate::features2d::Feature2D, cv_cuda_ORB_to_Feature2D }
	
	boxed_cast_base! { CUDA_ORB, crate::cudafeatures2d::CUDA_Feature2DAsync, cv_cuda_ORB_to_CUDA_Feature2DAsync }
	
	impl std::fmt::Debug for CUDA_ORB {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_ORB")
				.finish()
		}
	}
}

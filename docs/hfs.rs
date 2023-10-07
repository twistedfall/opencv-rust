pub mod hfs {
	//! # Hierarchical Feature Selection for Efficient Image Segmentation
	//! 
	//! The opencv hfs module contains an efficient algorithm to segment an image.
	//! This module is implemented based on the paper Hierarchical Feature Selection for Efficient
	//! Image Segmentation, ECCV 2016. The original project was developed by
	//! Yun Liu(<https://github.com/yun-liu/hfs>).
	//! 
	//! 
	//! Introduction to Hierarchical Feature Selection
	//! ----------------------------------------------
	//! 
	//! 
	//! This algorithm is executed in 3 stages:
	//! 
	//! In the first stage, the algorithm uses SLIC (simple linear iterative clustering) algorithm
	//! to obtain the superpixel of the input image.
	//! 
	//! In the second stage, the algorithm view each superpixel as a node in the graph.
	//! It will calculate a feature vector for each edge of the graph. It then calculates a weight
	//! for each edge based on the feature vector and trained SVM parameters. After obtaining
	//! weight for each edge, it will exploit  EGB (Efficient Graph-based Image Segmentation)
	//! algorithm to merge some nodes in the graph thus obtaining a coarser segmentation
	//! After these operations, a post process will be executed to merge regions that are smaller
	//! then a specific number of pixels into their nearby region.
	//! 
	//! In the third stage, the algorithm exploits the similar mechanism to further merge
	//! the small regions obtained in the second stage into even coarser segmentation.
	//! 
	//! After these three stages, we can obtain the final segmentation of the image.
	//! For further details about the algorithm, please refer to the original paper:
	//! Hierarchical Feature Selection for Efficient Image Segmentation, ECCV 2016
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::HfsSegmentTraitConst, super::HfsSegmentTrait };
	}
	
	/// Constant methods for [crate::hfs::HfsSegment]
	pub trait HfsSegmentTraitConst: core::AlgorithmTraitConst {
		fn as_raw_HfsSegment(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::hfs::HfsSegment]
	pub trait HfsSegmentTrait: core::AlgorithmTrait + crate::hfs::HfsSegmentTraitConst {
		fn as_raw_mut_HfsSegment(&mut self) -> *mut c_void;
	
		/// set and get the parameter segEgbThresholdI.
		/// This parameter is used in the second stage mentioned above.
		/// It is a constant used to threshold weights of the edge when merging
		/// adjacent nodes when applying EGB algorithm. The segmentation result
		/// tends to have more regions remained if this value is large and vice versa.
		#[inline]
		fn set_seg_egb_threshold_i(&mut self, c: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_setSegEgbThresholdI_float(self.as_raw_mut_HfsSegment(), c, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_seg_egb_threshold_i(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_getSegEgbThresholdI(self.as_raw_mut_HfsSegment(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set and get the parameter minRegionSizeI.
		/// This parameter is used in the second stage
		/// mentioned above. After the EGB segmentation, regions that have fewer
		/// pixels then this parameter will be merged into it's adjacent region.
		#[inline]
		fn set_min_region_size_i(&mut self, n: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_setMinRegionSizeI_int(self.as_raw_mut_HfsSegment(), n, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_region_size_i(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_getMinRegionSizeI(self.as_raw_mut_HfsSegment(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set and get the parameter segEgbThresholdII.
		/// This parameter is used in the third stage
		/// mentioned above. It serves the same purpose as segEgbThresholdI.
		/// The segmentation result tends to have more regions remained if
		/// this value is large and vice versa.
		#[inline]
		fn set_seg_egb_threshold_ii(&mut self, c: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_setSegEgbThresholdII_float(self.as_raw_mut_HfsSegment(), c, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_seg_egb_threshold_ii(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_getSegEgbThresholdII(self.as_raw_mut_HfsSegment(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set and get the parameter minRegionSizeII.
		/// This parameter is used in the third stage
		/// mentioned above. It serves the same purpose as minRegionSizeI
		#[inline]
		fn set_min_region_size_ii(&mut self, n: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_setMinRegionSizeII_int(self.as_raw_mut_HfsSegment(), n, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_min_region_size_ii(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_getMinRegionSizeII(self.as_raw_mut_HfsSegment(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set and get the parameter spatialWeight.
		/// This parameter is used in the first stage
		/// mentioned above(the SLIC stage). It describes how important is the role
		/// of position when calculating the distance between each pixel and it's
		/// center. The exact formula to calculate the distance is
		/// ![inline formula](https://latex.codecogs.com/png.latex?colorDistance%20%2B%20spatialWeight%20%5Ctimes%20spatialDistance).
		/// The segmentation result tends to have more local consistency
		/// if this value is larger.
		#[inline]
		fn set_spatial_weight(&mut self, w: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_setSpatialWeight_float(self.as_raw_mut_HfsSegment(), w, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_spatial_weight(&mut self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_getSpatialWeight(self.as_raw_mut_HfsSegment(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set and get the parameter slicSpixelSize.
		/// This parameter is used in the first stage mentioned
		/// above(the SLIC stage). It describes the size of each
		/// superpixel when initializing SLIC. Every superpixel
		/// approximately has ![inline formula](https://latex.codecogs.com/png.latex?slicSpixelSize%20%5Ctimes%20slicSpixelSize)
		/// pixels in the beginning.
		#[inline]
		fn set_slic_spixel_size(&mut self, n: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_setSlicSpixelSize_int(self.as_raw_mut_HfsSegment(), n, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_slic_spixel_size(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_getSlicSpixelSize(self.as_raw_mut_HfsSegment(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set and get the parameter numSlicIter.
		/// This parameter is used in the first stage. It
		/// describes how many iteration to perform when executing SLIC.
		#[inline]
		fn set_num_slic_iter(&mut self, n: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_setNumSlicIter_int(self.as_raw_mut_HfsSegment(), n, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_num_slic_iter(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_getNumSlicIter(self.as_raw_mut_HfsSegment(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// do segmentation gpu
		/// ## Parameters
		/// * src: : the input image
		/// * ifDraw: : if draw the image in the returned Mat. if this parameter is false,
		/// then the content of the returned Mat is a matrix of index, describing the region
		/// each pixel belongs to. And it's data type is CV_16U. If this parameter is true,
		/// then the returned Mat is a segmented picture, and color of each region is the
		/// average color of all pixels in that region. And it's data type is the same as
		/// the input image
		/// 
		/// ## C++ default parameters
		/// * if_draw: true
		#[inline]
		fn perform_segment_gpu(&mut self, src: &impl core::ToInputArray, if_draw: bool) -> Result<core::Mat> {
			input_array_arg!(src);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_performSegmentGpu_const__InputArrayR_bool(self.as_raw_mut_HfsSegment(), src.as_raw__InputArray(), if_draw, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// do segmentation gpu
		/// ## Parameters
		/// * src: : the input image
		/// * ifDraw: : if draw the image in the returned Mat. if this parameter is false,
		/// then the content of the returned Mat is a matrix of index, describing the region
		/// each pixel belongs to. And it's data type is CV_16U. If this parameter is true,
		/// then the returned Mat is a segmented picture, and color of each region is the
		/// average color of all pixels in that region. And it's data type is the same as
		/// the input image
		/// 
		/// ## Note
		/// This alternative version of [perform_segment_gpu] function uses the following default values for its arguments:
		/// * if_draw: true
		#[inline]
		fn perform_segment_gpu_def(&mut self, src: &impl core::ToInputArray) -> Result<core::Mat> {
			input_array_arg!(src);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_performSegmentGpu_const__InputArrayR(self.as_raw_mut_HfsSegment(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// do segmentation with cpu
		/// This method is only implemented for reference.
		/// It is highly NOT recommanded to use it.
		/// 
		/// ## C++ default parameters
		/// * if_draw: true
		#[inline]
		fn perform_segment_cpu(&mut self, src: &impl core::ToInputArray, if_draw: bool) -> Result<core::Mat> {
			input_array_arg!(src);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_performSegmentCpu_const__InputArrayR_bool(self.as_raw_mut_HfsSegment(), src.as_raw__InputArray(), if_draw, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// do segmentation with cpu
		/// This method is only implemented for reference.
		/// It is highly NOT recommanded to use it.
		/// 
		/// ## Note
		/// This alternative version of [perform_segment_cpu] function uses the following default values for its arguments:
		/// * if_draw: true
		#[inline]
		fn perform_segment_cpu_def(&mut self, src: &impl core::ToInputArray) -> Result<core::Mat> {
			input_array_arg!(src);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_performSegmentCpu_const__InputArrayR(self.as_raw_mut_HfsSegment(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	pub struct HfsSegment {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { HfsSegment }
	
	impl Drop for HfsSegment {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_hfs_HfsSegment_delete(self.as_raw_mut_HfsSegment()) };
		}
	}
	
	unsafe impl Send for HfsSegment {}
	
	impl core::AlgorithmTraitConst for HfsSegment {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for HfsSegment {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::hfs::HfsSegmentTraitConst for HfsSegment {
		#[inline] fn as_raw_HfsSegment(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::hfs::HfsSegmentTrait for HfsSegment {
		#[inline] fn as_raw_mut_HfsSegment(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl HfsSegment {
		/// create a hfs object
		/// ## Parameters
		/// * height: : the height of the input image
		/// * width: : the width of the input image
		/// * segEgbThresholdI: : parameter segEgbThresholdI
		/// * minRegionSizeI: : parameter minRegionSizeI
		/// * segEgbThresholdII: : parameter segEgbThresholdII
		/// * minRegionSizeII: : parameter minRegionSizeII
		/// * spatialWeight: : parameter spatialWeight
		/// * slicSpixelSize: : parameter slicSpixelSize
		/// * numSlicIter: : parameter numSlicIter
		/// 
		/// ## C++ default parameters
		/// * seg_egb_threshold_i: 0.08f
		/// * min_region_size_i: 100
		/// * seg_egb_threshold_ii: 0.28f
		/// * min_region_size_ii: 200
		/// * spatial_weight: 0.6f
		/// * slic_spixel_size: 8
		/// * num_slic_iter: 5
		#[inline]
		pub fn create(height: i32, width: i32, seg_egb_threshold_i: f32, min_region_size_i: i32, seg_egb_threshold_ii: f32, min_region_size_ii: i32, spatial_weight: f32, slic_spixel_size: i32, num_slic_iter: i32) -> Result<core::Ptr<crate::hfs::HfsSegment>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_create_int_int_float_int_float_int_float_int_int(height, width, seg_egb_threshold_i, min_region_size_i, seg_egb_threshold_ii, min_region_size_ii, spatial_weight, slic_spixel_size, num_slic_iter, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::hfs::HfsSegment>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// create a hfs object
		/// ## Parameters
		/// * height: : the height of the input image
		/// * width: : the width of the input image
		/// * segEgbThresholdI: : parameter segEgbThresholdI
		/// * minRegionSizeI: : parameter minRegionSizeI
		/// * segEgbThresholdII: : parameter segEgbThresholdII
		/// * minRegionSizeII: : parameter minRegionSizeII
		/// * spatialWeight: : parameter spatialWeight
		/// * slicSpixelSize: : parameter slicSpixelSize
		/// * numSlicIter: : parameter numSlicIter
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * seg_egb_threshold_i: 0.08f
		/// * min_region_size_i: 100
		/// * seg_egb_threshold_ii: 0.28f
		/// * min_region_size_ii: 200
		/// * spatial_weight: 0.6f
		/// * slic_spixel_size: 8
		/// * num_slic_iter: 5
		#[inline]
		pub fn create_def(height: i32, width: i32) -> Result<core::Ptr<crate::hfs::HfsSegment>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_hfs_HfsSegment_create_int_int(height, width, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::hfs::HfsSegment>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { HfsSegment, core::Algorithm, cv_hfs_HfsSegment_to_Algorithm }
	
	impl std::fmt::Debug for HfsSegment {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("HfsSegment")
				.finish()
		}
	}
}

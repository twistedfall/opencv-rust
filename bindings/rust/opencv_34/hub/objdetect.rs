#![allow(unused_parens)]
//! # Object Detection
//! 
//! Haar Feature-based Cascade Classifier for Object Detection
//! ----------------------------------------------------------
//! 
//! The object detector described below has been initially proposed by Paul Viola [Viola01](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Viola01) and
//! improved by Rainer Lienhart [Lienhart02](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Lienhart02) .
//! 
//! First, a classifier (namely a *cascade of boosted classifiers working with haar-like features*) is
//! trained with a few hundred sample views of a particular object (i.e., a face or a car), called
//! positive examples, that are scaled to the same size (say, 20x20), and negative examples - arbitrary
//! images of the same size.
//! 
//! After a classifier is trained, it can be applied to a region of interest (of the same size as used
//! during the training) in an input image. The classifier outputs a "1" if the region is likely to show
//! the object (i.e., face/car), and "0" otherwise. To search for the object in the whole image one can
//! move the search window across the image and check every location using the classifier. The
//! classifier is designed so that it can be easily "resized" in order to be able to find the objects of
//! interest at different sizes, which is more efficient than resizing the image itself. So, to find an
//! object of an unknown size in the image the scan procedure should be done several times at different
//! scales.
//! 
//! The word "cascade" in the classifier name means that the resultant classifier consists of several
//! simpler classifiers (*stages*) that are applied subsequently to a region of interest until at some
//! stage the candidate is rejected or all the stages are passed. The word "boosted" means that the
//! classifiers at every stage of the cascade are complex themselves and they are built out of basic
//! classifiers using one of four different boosting techniques (weighted voting). Currently Discrete
//! Adaboost, Real Adaboost, Gentle Adaboost and Logitboost are supported. The basic classifiers are
//! decision-tree classifiers with at least 2 leaves. Haar-like features are the input to the basic
//! classifiers, and are calculated as described below. The current algorithm uses the following
//! Haar-like features:
//! 
//! ![image](https://docs.opencv.org/3.4.10/haarfeatures.png)
//! 
//! The feature used in a particular classifier is specified by its shape (1a, 2b etc.), position within
//! the region of interest and the scale (this scale is not the same as the scale used at the detection
//! stage, though these two scales are multiplied). For example, in the case of the third line feature
//! (2c) the response is calculated as the difference between the sum of image pixels under the
//! rectangle covering the whole feature (including the two white stripes and the black stripe in the
//! middle) and the sum of the image pixels under the black stripe multiplied by 3 in order to
//! compensate for the differences in the size of areas. The sums of pixel values over a rectangular
//! regions are calculated rapidly using integral images (see below and the integral description).
//! 
//! To see the object detector at work, have a look at the facedetect demo:
//! <https://github.com/opencv/opencv/tree/3.4/samples/cpp/dbt_face_detection.cpp>
//! 
//! The following reference is for the detection part only. There is a separate application called
//! opencv_traincascade that can train a cascade of boosted classifiers from a set of samples.
//! 
//! 
//! Note: In the new C++ interface it is also possible to use LBP (local binary pattern) features in
//! addition to Haar-like features. .. [Viola01] Paul Viola and Michael J. Jones. Rapid Object Detection
//! using a Boosted Cascade of Simple Features. IEEE CVPR, 2001. The paper is available online at
//! <http://research.microsoft.com/en-us/um/people/viola/Pubs/Detect/violaJones_CVPR2001.pdf>
//!    # C API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::SimilarRectsTrait, super::BaseCascadeClassifier_MaskGenerator, super::BaseCascadeClassifier, super::CascadeClassifierTrait, super::DetectionROITrait, super::HOGDescriptorTrait, super::QRCodeDetectorTrait, super::DetectionBasedTracker_ParametersTrait, super::DetectionBasedTracker_IDetector, super::DetectionBasedTracker_ExtObjectTrait, super::DetectionBasedTrackerTrait };
}

pub const CASCADE_DO_CANNY_PRUNING: i32 = 1;
pub const CASCADE_DO_ROUGH_SEARCH: i32 = 8;
pub const CASCADE_FIND_BIGGEST_OBJECT: i32 = 4;
pub const CASCADE_SCALE_IMAGE: i32 = 2;
pub const CV_HAAR_DO_CANNY_PRUNING: i32 = 1;
pub const CV_HAAR_DO_ROUGH_SEARCH: i32 = 8;
pub const CV_HAAR_FEATURE_MAX: i32 = 3;
pub const CV_HAAR_FIND_BIGGEST_OBJECT: i32 = 4;
pub const CV_HAAR_MAGIC_VAL: i32 = 0x42500000;
pub const CV_HAAR_SCALE_IMAGE: i32 = 2;
pub const CV_HAAR_STAGE_MAX: i32 = 1000;
pub const CV_TYPE_NAME_HAAR: &'static str = "opencv-haar-classifier";
/// Default nlevels value.
pub const HOGDescriptor_DEFAULT_NLEVELS: i32 = 64;
/// Default histogramNormType
pub const HOGDescriptor_L2Hys: i32 = 0;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DetectionBasedTracker_ObjectStatus {
	DETECTED_NOT_SHOWN_YET = 0,
	DETECTED = 1,
	DETECTED_TEMPORARY_LOST = 2,
	WRONG_OBJECT = 3,
}

opencv_type_enum! { crate::objdetect::DetectionBasedTracker_ObjectStatus }

pub fn create_face_detection_mask_generator() -> Result<core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>> {
	unsafe { sys::cv_createFaceDetectionMaskGenerator() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>::opencv_from_extern(r) } )
}

/// Decode QR code in image and return text that is encrypted in QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Input vector of vertices of a quadrangle of minimal area that describes QR code.
/// * decoded_info: String information that is encrypted in QR code.
/// * straight_qrcode: Matrix of the type CV_8UC1 containing an binary straight QR code.
/// 
/// ## C++ default parameters
/// * straight_qrcode: noArray()
pub fn decode_qr_code(in_: &dyn core::ToInputArray, points: &dyn core::ToInputArray, decoded_info: &mut String, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<bool> {
	input_array_arg!(in_);
	input_array_arg!(points);
	string_arg_output_send!(via decoded_info_via);
	output_array_arg!(straight_qrcode);
	let out = unsafe { sys::cv_decodeQRCode_const__InputArrayR_const__InputArrayR_stringR_const__OutputArrayR(in_.as_raw__InputArray(), points.as_raw__InputArray(), &mut decoded_info_via, straight_qrcode.as_raw__OutputArray()) }.into_result();
	string_arg_output_receive!(out, decoded_info_via => decoded_info);
	out
}

/// Detect QR code in image and return minimum area of quadrangle that describes QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Output vector of vertices of a quadrangle of minimal area that describes QR code.
/// * eps_x: Epsilon neighborhood, which allows you to determine the horizontal pattern of the scheme 1:1:3:1:1 according to QR code standard.
/// * eps_y: Epsilon neighborhood, which allows you to determine the vertical pattern of the scheme 1:1:3:1:1 according to QR code standard.
/// 
/// ## C++ default parameters
/// * eps_x: 0.2
/// * eps_y: 0.1
pub fn detect_qr_code(in_: &dyn core::ToInputArray, points: &mut core::Vector::<core::Point>, eps_x: f64, eps_y: f64) -> Result<bool> {
	input_array_arg!(in_);
	unsafe { sys::cv_detectQRCode_const__InputArrayR_vector_Point_R_double_double(in_.as_raw__InputArray(), points.as_raw_mut_VectorOfPoint(), eps_x, eps_y) }.into_result()
}

/// ## C++ default parameters
/// * detect_threshold: 0.0
/// * win_det_size: Size(64,128)
pub fn group_rectangles_meanshift(rect_list: &mut core::Vector::<core::Rect>, found_weights: &mut core::Vector::<f64>, found_scales: &mut core::Vector::<f64>, detect_threshold: f64, win_det_size: core::Size) -> Result<()> {
	unsafe { sys::cv_groupRectangles_meanshift_vector_Rect_R_vector_double_R_vector_double_R_double_Size(rect_list.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), found_scales.as_raw_mut_VectorOff64(), detect_threshold, win_det_size.opencv_as_extern()) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## C++ default parameters
/// * eps: 0.2
pub fn group_rectangles(rect_list: &mut core::Vector::<core::Rect>, group_threshold: i32, eps: f64) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_R_int_double(rect_list.as_raw_mut_VectorOfRect(), group_threshold, eps) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## Overloaded parameters
pub fn group_rectangles_levelweights(rect_list: &mut core::Vector::<core::Rect>, group_threshold: i32, eps: f64, weights: &mut core::Vector::<i32>, level_weights: &mut core::Vector::<f64>) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_R_int_double_vector_int_X_vector_double_X(rect_list.as_raw_mut_VectorOfRect(), group_threshold, eps, weights.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64()) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * eps: 0.2
pub fn group_rectangles_weights(rect_list: &mut core::Vector::<core::Rect>, weights: &mut core::Vector::<i32>, group_threshold: i32, eps: f64) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_R_vector_int_R_int_double(rect_list.as_raw_mut_VectorOfRect(), weights.as_raw_mut_VectorOfi32(), group_threshold, eps) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * eps: 0.2
pub fn group_rectangles_levels(rect_list: &mut core::Vector::<core::Rect>, reject_levels: &mut core::Vector::<i32>, level_weights: &mut core::Vector::<f64>, group_threshold: i32, eps: f64) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_R_vector_int_R_vector_double_R_int_double(rect_list.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), group_threshold, eps) }.into_result()
}

pub trait BaseCascadeClassifier: core::AlgorithmTrait {
	fn as_raw_BaseCascadeClassifier(&self) -> *const c_void;
	fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void;

	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_BaseCascadeClassifier_empty_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn load(&mut self, filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		unsafe { sys::cv_BaseCascadeClassifier_load_const_StringR(self.as_raw_mut_BaseCascadeClassifier(), filename.opencv_as_extern()) }.into_result()
	}
	
	fn detect_multi_scale(&mut self, image: &dyn core::ToInputArray, objects: &mut core::Vector::<core::Rect>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern()) }.into_result()
	}
	
	fn detect_multi_scale_num(&mut self, image: &dyn core::ToInputArray, objects: &mut core::Vector::<core::Rect>, num_detections: &mut core::Vector::<i32>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), num_detections.as_raw_mut_VectorOfi32(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern()) }.into_result()
	}
	
	fn detect_multi_scale_levels(&mut self, image: &dyn core::ToInputArray, objects: &mut core::Vector::<core::Rect>, reject_levels: &mut core::Vector::<i32>, level_weights: &mut core::Vector::<f64>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(self.as_raw_mut_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), output_reject_levels) }.into_result()
	}
	
	fn is_old_format_cascade(&self) -> Result<bool> {
		unsafe { sys::cv_BaseCascadeClassifier_isOldFormatCascade_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn get_original_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_BaseCascadeClassifier_getOriginalWindowSize_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn get_feature_type(&self) -> Result<i32> {
		unsafe { sys::cv_BaseCascadeClassifier_getFeatureType_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn get_old_cascade(&mut self) -> Result<*mut c_void> {
		unsafe { sys::cv_BaseCascadeClassifier_getOldCascade(self.as_raw_mut_BaseCascadeClassifier()) }.into_result()
	}
	
	fn set_mask_generator(&mut self, mask_generator: &core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>) -> Result<()> {
		unsafe { sys::cv_BaseCascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(self.as_raw_mut_BaseCascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator()) }.into_result()
	}
	
	fn get_mask_generator(&mut self) -> Result<core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>> {
		unsafe { sys::cv_BaseCascadeClassifier_getMaskGenerator(self.as_raw_mut_BaseCascadeClassifier()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>::opencv_from_extern(r) } )
	}
	
}

pub trait BaseCascadeClassifier_MaskGenerator {
	fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void;
	fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void;

	fn generate_mask(&mut self, src: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(self.as_raw_mut_BaseCascadeClassifier_MaskGenerator(), src.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn initialize_mask(&mut self, unnamed: &core::Mat) -> Result<()> {
		unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(self.as_raw_mut_BaseCascadeClassifier_MaskGenerator(), unnamed.as_raw_Mat()) }.into_result()
	}
	
}

/// @example samples/cpp/facedetect.cpp
/// This program demonstrates usage of the Cascade classifier class
/// \image html Cascade_Classifier_Tutorial_Result_Haar.jpg "Sample screenshot" width=321 height=254
/// 
/// Cascade classifier class for object detection.
pub trait CascadeClassifierTrait {
	fn as_raw_CascadeClassifier(&self) -> *const c_void;
	fn as_raw_mut_CascadeClassifier(&mut self) -> *mut c_void;

	fn cc(&mut self) -> core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier> {
		unsafe { sys::cv_CascadeClassifier_getPropCc(self.as_raw_mut_CascadeClassifier()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier>::opencv_from_extern(r) } ).expect("Infallible function failed: cc")
	}
	
	fn set_cc(&mut self, mut val: core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier>) -> () {
		unsafe { sys::cv_CascadeClassifier_setPropCc_Ptr_BaseCascadeClassifier_(self.as_raw_mut_CascadeClassifier(), val.as_raw_mut_PtrOfBaseCascadeClassifier()) }.into_result().expect("Infallible function failed: set_cc")
	}
	
	/// Checks whether the classifier has been loaded.
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_CascadeClassifier_empty_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	/// Loads a classifier from a file.
	/// 
	/// ## Parameters
	/// * filename: Name of the file from which the classifier is loaded. The file may contain an old
	/// HAAR classifier trained by the haartraining application or a new cascade classifier trained by the
	/// traincascade application.
	fn load(&mut self, filename: &str) -> Result<bool> {
		extern_container_arg!(filename);
		unsafe { sys::cv_CascadeClassifier_load_const_StringR(self.as_raw_mut_CascadeClassifier(), filename.opencv_as_extern()) }.into_result()
	}
	
	/// Reads a classifier from a FileStorage node.
	/// 
	/// 
	/// Note: The file may contain a new cascade classifier (trained traincascade application) only.
	fn read(&mut self, node: &core::FileNode) -> Result<bool> {
		unsafe { sys::cv_CascadeClassifier_read_const_FileNodeR(self.as_raw_mut_CascadeClassifier(), node.as_raw_FileNode()) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// 
	/// ## Parameters
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	/// rectangles may be partially outside the original image.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	/// to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	/// cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// The function is parallelized with the TBB library.
	/// 
	/// 
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	/// 
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	fn detect_multi_scale(&mut self, image: &dyn core::ToInputArray, objects: &mut core::Vector::<core::Rect>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern()) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// 
	/// ## Parameters
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	/// rectangles may be partially outside the original image.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	/// to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	/// cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// The function is parallelized with the TBB library.
	/// 
	/// 
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	/// 
	/// ## Overloaded parameters
	/// 
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	///    rectangles may be partially outside the original image.
	/// * numDetections: Vector of detection numbers for the corresponding objects. An object's number
	///    of detections is the number of neighboring positively classified rectangles that were joined
	///    together to form the object.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	///    to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	///    cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	fn detect_multi_scale2(&mut self, image: &dyn core::ToInputArray, objects: &mut core::Vector::<core::Rect>, num_detections: &mut core::Vector::<i32>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), num_detections.as_raw_mut_VectorOfi32(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern()) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// 
	/// ## Parameters
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	/// rectangles may be partially outside the original image.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	/// to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	/// cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// The function is parallelized with the TBB library.
	/// 
	/// 
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	/// 
	/// ## Overloaded parameters
	/// 
	///    This function allows you to retrieve the final stage decision certainty of classification.
	///    For this, one needs to set `outputRejectLevels` on true and provide the `rejectLevels` and `levelWeights` parameter.
	///    For each resulting detection, `levelWeights` will then contain the certainty of classification at the final stage.
	///    This value can then be used to separate strong from weaker classifications.
	/// 
	///    A code sample on how to use it efficiently can be found below:
	///    ```ignore
	///    Mat img;
	///    vector<double> weights;
	///    vector<int> levels;
	///    vector<Rect> detections;
	///    CascadeClassifier model("/path/to/your/model.xml");
	///    model.detectMultiScale(img, detections, levels, weights, 1.1, 3, 0, Size(), Size(), true);
	///    cerr << "Detection " << detections[0] << " with weight " << weights[0] << endl;
	///    ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	/// * output_reject_levels: false
	fn detect_multi_scale3(&mut self, image: &dyn core::ToInputArray, objects: &mut core::Vector::<core::Rect>, reject_levels: &mut core::Vector::<i32>, level_weights: &mut core::Vector::<f64>, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(self.as_raw_mut_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_mut_VectorOfRect(), reject_levels.as_raw_mut_VectorOfi32(), level_weights.as_raw_mut_VectorOff64(), scale_factor, min_neighbors, flags, min_size.opencv_as_extern(), max_size.opencv_as_extern(), output_reject_levels) }.into_result()
	}
	
	fn is_old_format_cascade(&self) -> Result<bool> {
		unsafe { sys::cv_CascadeClassifier_isOldFormatCascade_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	fn get_original_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_CascadeClassifier_getOriginalWindowSize_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	fn get_feature_type(&self) -> Result<i32> {
		unsafe { sys::cv_CascadeClassifier_getFeatureType_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	fn get_old_cascade(&mut self) -> Result<*mut c_void> {
		unsafe { sys::cv_CascadeClassifier_getOldCascade(self.as_raw_mut_CascadeClassifier()) }.into_result()
	}
	
	fn set_mask_generator(&mut self, mask_generator: &core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>) -> Result<()> {
		unsafe { sys::cv_CascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(self.as_raw_mut_CascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator()) }.into_result()
	}
	
	fn get_mask_generator(&mut self) -> Result<core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>> {
		unsafe { sys::cv_CascadeClassifier_getMaskGenerator(self.as_raw_mut_CascadeClassifier()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>::opencv_from_extern(r) } )
	}
	
}

/// @example samples/cpp/facedetect.cpp
/// This program demonstrates usage of the Cascade classifier class
/// \image html Cascade_Classifier_Tutorial_Result_Haar.jpg "Sample screenshot" width=321 height=254
/// 
/// Cascade classifier class for object detection.
pub struct CascadeClassifier {
	ptr: *mut c_void
}

opencv_type_boxed! { CascadeClassifier }

impl Drop for CascadeClassifier {
	fn drop(&mut self) {
		extern "C" { fn cv_CascadeClassifier_delete(instance: *mut c_void); }
		unsafe { cv_CascadeClassifier_delete(self.as_raw_mut_CascadeClassifier()) };
	}
}

impl CascadeClassifier {
	#[inline] pub fn as_raw_CascadeClassifier(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CascadeClassifier {}

impl crate::objdetect::CascadeClassifierTrait for CascadeClassifier {
	#[inline] fn as_raw_CascadeClassifier(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CascadeClassifier {
	pub fn default() -> Result<crate::objdetect::CascadeClassifier> {
		unsafe { sys::cv_CascadeClassifier_CascadeClassifier() }.into_result().map(|r| unsafe { crate::objdetect::CascadeClassifier::opencv_from_extern(r) } )
	}
	
	/// Loads a classifier from a file.
	/// 
	/// ## Parameters
	/// * filename: Name of the file from which the classifier is loaded.
	pub fn new(filename: &str) -> Result<crate::objdetect::CascadeClassifier> {
		extern_container_arg!(filename);
		unsafe { sys::cv_CascadeClassifier_CascadeClassifier_const_StringR(filename.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::objdetect::CascadeClassifier::opencv_from_extern(r) } )
	}
	
	pub fn convert(oldcascade: &str, newcascade: &str) -> Result<bool> {
		extern_container_arg!(oldcascade);
		extern_container_arg!(newcascade);
		unsafe { sys::cv_CascadeClassifier_convert_const_StringR_const_StringR(oldcascade.opencv_as_extern(), newcascade.opencv_as_extern()) }.into_result()
	}
	
}

pub trait DetectionBasedTrackerTrait {
	fn as_raw_DetectionBasedTracker(&self) -> *const c_void;
	fn as_raw_mut_DetectionBasedTracker(&mut self) -> *mut c_void;

	fn run(&mut self) -> Result<bool> {
		unsafe { sys::cv_DetectionBasedTracker_run(self.as_raw_mut_DetectionBasedTracker()) }.into_result()
	}
	
	fn stop(&mut self) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_stop(self.as_raw_mut_DetectionBasedTracker()) }.into_result()
	}
	
	fn reset_tracking(&mut self) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_resetTracking(self.as_raw_mut_DetectionBasedTracker()) }.into_result()
	}
	
	fn process(&mut self, image_gray: &core::Mat) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_process_const_MatR(self.as_raw_mut_DetectionBasedTracker(), image_gray.as_raw_Mat()) }.into_result()
	}
	
	fn set_parameters(&mut self, params: &crate::objdetect::DetectionBasedTracker_Parameters) -> Result<bool> {
		unsafe { sys::cv_DetectionBasedTracker_setParameters_const_ParametersR(self.as_raw_mut_DetectionBasedTracker(), params.as_raw_DetectionBasedTracker_Parameters()) }.into_result()
	}
	
	fn get_parameters(&self) -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
		unsafe { sys::cv_DetectionBasedTracker_getParameters_const(self.as_raw_DetectionBasedTracker()) }.into_result().map(|r| unsafe { crate::objdetect::DetectionBasedTracker_Parameters::opencv_from_extern(r) } )
	}
	
	fn get_objects(&self, result: &mut core::Vector::<core::Rect>) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vector_Rect_R(self.as_raw_DetectionBasedTracker(), result.as_raw_mut_VectorOfRect()) }.into_result()
	}
	
	fn get_objects_1(&self, result: &mut core::Vector::<crate::objdetect::DetectionBasedTracker_ExtObject>) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vector_ExtObject_R(self.as_raw_DetectionBasedTracker(), result.as_raw_mut_VectorOfDetectionBasedTracker_ExtObject()) }.into_result()
	}
	
	fn add_object(&mut self, location: core::Rect) -> Result<i32> {
		unsafe { sys::cv_DetectionBasedTracker_addObject_const_RectR(self.as_raw_mut_DetectionBasedTracker(), &location) }.into_result()
	}
	
}

pub struct DetectionBasedTracker {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectionBasedTracker }

impl Drop for DetectionBasedTracker {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionBasedTracker_delete(instance: *mut c_void); }
		unsafe { cv_DetectionBasedTracker_delete(self.as_raw_mut_DetectionBasedTracker()) };
	}
}

impl DetectionBasedTracker {
	#[inline] pub fn as_raw_DetectionBasedTracker(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_DetectionBasedTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DetectionBasedTracker {}

impl crate::objdetect::DetectionBasedTrackerTrait for DetectionBasedTracker {
	#[inline] fn as_raw_DetectionBasedTracker(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DetectionBasedTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectionBasedTracker {
	pub fn new(mut main_detector: core::Ptr::<dyn crate::objdetect::DetectionBasedTracker_IDetector>, mut tracking_detector: core::Ptr::<dyn crate::objdetect::DetectionBasedTracker_IDetector>, params: &crate::objdetect::DetectionBasedTracker_Parameters) -> Result<crate::objdetect::DetectionBasedTracker> {
		unsafe { sys::cv_DetectionBasedTracker_DetectionBasedTracker_Ptr_IDetector__Ptr_IDetector__const_ParametersR(main_detector.as_raw_mut_PtrOfDetectionBasedTracker_IDetector(), tracking_detector.as_raw_mut_PtrOfDetectionBasedTracker_IDetector(), params.as_raw_DetectionBasedTracker_Parameters()) }.into_result().map(|r| unsafe { crate::objdetect::DetectionBasedTracker::opencv_from_extern(r) } )
	}
	
}

pub trait DetectionBasedTracker_ExtObjectTrait {
	fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *const c_void;
	fn as_raw_mut_DetectionBasedTracker_ExtObject(&mut self) -> *mut c_void;

	fn id(&self) -> i32 {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_getPropId_const(self.as_raw_DetectionBasedTracker_ExtObject()) }.into_result().expect("Infallible function failed: id")
	}
	
	fn set_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_setPropId_int(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val) }.into_result().expect("Infallible function failed: set_id")
	}
	
	fn location(&self) -> core::Rect {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_getPropLocation_const(self.as_raw_DetectionBasedTracker_ExtObject()) }.into_result().expect("Infallible function failed: location")
	}
	
	fn set_location(&mut self, val: core::Rect) -> () {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_setPropLocation_Rect(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_location")
	}
	
	fn status(&self) -> crate::objdetect::DetectionBasedTracker_ObjectStatus {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_getPropStatus_const(self.as_raw_DetectionBasedTracker_ExtObject()) }.into_result().expect("Infallible function failed: status")
	}
	
	fn set_status(&mut self, val: crate::objdetect::DetectionBasedTracker_ObjectStatus) -> () {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_setPropStatus_ObjectStatus(self.as_raw_mut_DetectionBasedTracker_ExtObject(), val) }.into_result().expect("Infallible function failed: set_status")
	}
	
}

pub struct DetectionBasedTracker_ExtObject {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectionBasedTracker_ExtObject }

impl Drop for DetectionBasedTracker_ExtObject {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionBasedTracker_ExtObject_delete(instance: *mut c_void); }
		unsafe { cv_DetectionBasedTracker_ExtObject_delete(self.as_raw_mut_DetectionBasedTracker_ExtObject()) };
	}
}

impl DetectionBasedTracker_ExtObject {
	#[inline] pub fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_DetectionBasedTracker_ExtObject(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DetectionBasedTracker_ExtObject {}

impl crate::objdetect::DetectionBasedTracker_ExtObjectTrait for DetectionBasedTracker_ExtObject {
	#[inline] fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DetectionBasedTracker_ExtObject(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectionBasedTracker_ExtObject {
	pub fn new(_id: i32, _location: core::Rect, _status: crate::objdetect::DetectionBasedTracker_ObjectStatus) -> Result<crate::objdetect::DetectionBasedTracker_ExtObject> {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(_id, _location.opencv_as_extern(), _status) }.into_result().map(|r| unsafe { crate::objdetect::DetectionBasedTracker_ExtObject::opencv_from_extern(r) } )
	}
	
}

pub trait DetectionBasedTracker_IDetector {
	fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void;
	fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void;

	fn detect(&mut self, image: &core::Mat, objects: &mut core::Vector::<core::Rect>) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_detect_const_MatR_vector_Rect_R(self.as_raw_mut_DetectionBasedTracker_IDetector(), image.as_raw_Mat(), objects.as_raw_mut_VectorOfRect()) }.into_result()
	}
	
	fn set_min_object_size(&mut self, min: core::Size) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(self.as_raw_mut_DetectionBasedTracker_IDetector(), &min) }.into_result()
	}
	
	fn set_max_object_size(&mut self, max: core::Size) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(self.as_raw_mut_DetectionBasedTracker_IDetector(), &max) }.into_result()
	}
	
	fn get_min_object_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn get_max_object_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn get_scale_factor(&mut self) -> Result<f32> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getScaleFactor(self.as_raw_mut_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn set_scale_factor(&mut self, value: f32) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setScaleFactor_float(self.as_raw_mut_DetectionBasedTracker_IDetector(), value) }.into_result()
	}
	
	fn get_min_neighbours(&mut self) -> Result<i32> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinNeighbours(self.as_raw_mut_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn set_min_neighbours(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(self.as_raw_mut_DetectionBasedTracker_IDetector(), value) }.into_result()
	}
	
}

pub trait DetectionBasedTracker_ParametersTrait {
	fn as_raw_DetectionBasedTracker_Parameters(&self) -> *const c_void;
	fn as_raw_mut_DetectionBasedTracker_Parameters(&mut self) -> *mut c_void;

	fn max_track_lifetime(&self) -> i32 {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_getPropMaxTrackLifetime_const(self.as_raw_DetectionBasedTracker_Parameters()) }.into_result().expect("Infallible function failed: max_track_lifetime")
	}
	
	fn set_max_track_lifetime(&mut self, val: i32) -> () {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_setPropMaxTrackLifetime_int(self.as_raw_mut_DetectionBasedTracker_Parameters(), val) }.into_result().expect("Infallible function failed: set_max_track_lifetime")
	}
	
	fn min_detection_period(&self) -> i32 {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_getPropMinDetectionPeriod_const(self.as_raw_DetectionBasedTracker_Parameters()) }.into_result().expect("Infallible function failed: min_detection_period")
	}
	
	fn set_min_detection_period(&mut self, val: i32) -> () {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_setPropMinDetectionPeriod_int(self.as_raw_mut_DetectionBasedTracker_Parameters(), val) }.into_result().expect("Infallible function failed: set_min_detection_period")
	}
	
}

pub struct DetectionBasedTracker_Parameters {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectionBasedTracker_Parameters }

impl Drop for DetectionBasedTracker_Parameters {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionBasedTracker_Parameters_delete(instance: *mut c_void); }
		unsafe { cv_DetectionBasedTracker_Parameters_delete(self.as_raw_mut_DetectionBasedTracker_Parameters()) };
	}
}

impl DetectionBasedTracker_Parameters {
	#[inline] pub fn as_raw_DetectionBasedTracker_Parameters(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_DetectionBasedTracker_Parameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DetectionBasedTracker_Parameters {}

impl crate::objdetect::DetectionBasedTracker_ParametersTrait for DetectionBasedTracker_Parameters {
	#[inline] fn as_raw_DetectionBasedTracker_Parameters(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DetectionBasedTracker_Parameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectionBasedTracker_Parameters {
	pub fn default() -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_Parameters() }.into_result().map(|r| unsafe { crate::objdetect::DetectionBasedTracker_Parameters::opencv_from_extern(r) } )
	}
	
}

/// struct for detection region of interest (ROI)
pub trait DetectionROITrait {
	fn as_raw_DetectionROI(&self) -> *const c_void;
	fn as_raw_mut_DetectionROI(&mut self) -> *mut c_void;

	/// scale(size) of the bounding box
	fn scale(&self) -> f64 {
		unsafe { sys::cv_DetectionROI_getPropScale_const(self.as_raw_DetectionROI()) }.into_result().expect("Infallible function failed: scale")
	}
	
	/// scale(size) of the bounding box
	fn set_scale(&mut self, val: f64) -> () {
		unsafe { sys::cv_DetectionROI_setPropScale_double(self.as_raw_mut_DetectionROI(), val) }.into_result().expect("Infallible function failed: set_scale")
	}
	
	/// set of requested locations to be evaluated
	fn locations(&mut self) -> core::Vector::<core::Point> {
		unsafe { sys::cv_DetectionROI_getPropLocations(self.as_raw_mut_DetectionROI()) }.into_result().map(|r| unsafe { core::Vector::<core::Point>::opencv_from_extern(r) } ).expect("Infallible function failed: locations")
	}
	
	/// set of requested locations to be evaluated
	fn set_locations(&mut self, mut val: core::Vector::<core::Point>) -> () {
		unsafe { sys::cv_DetectionROI_setPropLocations_vector_Point_(self.as_raw_mut_DetectionROI(), val.as_raw_mut_VectorOfPoint()) }.into_result().expect("Infallible function failed: set_locations")
	}
	
	/// vector that will contain confidence values for each location
	fn confidences(&mut self) -> core::Vector::<f64> {
		unsafe { sys::cv_DetectionROI_getPropConfidences(self.as_raw_mut_DetectionROI()) }.into_result().map(|r| unsafe { core::Vector::<f64>::opencv_from_extern(r) } ).expect("Infallible function failed: confidences")
	}
	
	/// vector that will contain confidence values for each location
	fn set_confidences(&mut self, mut val: core::Vector::<f64>) -> () {
		unsafe { sys::cv_DetectionROI_setPropConfidences_vector_double_(self.as_raw_mut_DetectionROI(), val.as_raw_mut_VectorOff64()) }.into_result().expect("Infallible function failed: set_confidences")
	}
	
}

/// struct for detection region of interest (ROI)
pub struct DetectionROI {
	ptr: *mut c_void
}

opencv_type_boxed! { DetectionROI }

impl Drop for DetectionROI {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionROI_delete(instance: *mut c_void); }
		unsafe { cv_DetectionROI_delete(self.as_raw_mut_DetectionROI()) };
	}
}

impl DetectionROI {
	#[inline] pub fn as_raw_DetectionROI(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_DetectionROI(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DetectionROI {}

impl crate::objdetect::DetectionROITrait for DetectionROI {
	#[inline] fn as_raw_DetectionROI(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DetectionROI(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DetectionROI {
}

/// Implementation of HOG (Histogram of Oriented Gradients) descriptor and object detector.
/// 
/// the HOG descriptor algorithm introduced by Navneet Dalal and Bill Triggs [Dalal2005](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Dalal2005) .
/// 
/// useful links:
/// 
/// https://hal.inria.fr/inria-00548512/document/
/// 
/// https://en.wikipedia.org/wiki/Histogram_of_oriented_gradients
/// 
/// https://software.intel.com/en-us/ipp-dev-reference-histogram-of-oriented-gradients-hog-descriptor
/// 
/// http://www.learnopencv.com/histogram-of-oriented-gradients
/// 
/// http://www.learnopencv.com/handwritten-digits-classification-an-opencv-c-python-tutorial
pub trait HOGDescriptorTrait {
	fn as_raw_HOGDescriptor(&self) -> *const c_void;
	fn as_raw_mut_HOGDescriptor(&mut self) -> *mut c_void;

	/// Detection window size. Align to block size and block stride. Default value is Size(64,128).
	fn win_size(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_getPropWinSize_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: win_size")
	}
	
	/// Detection window size. Align to block size and block stride. Default value is Size(64,128).
	fn set_win_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropWinSize_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_win_size")
	}
	
	/// Block size in pixels. Align to cell size. Default value is Size(16,16).
	fn block_size(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_getPropBlockSize_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: block_size")
	}
	
	/// Block size in pixels. Align to cell size. Default value is Size(16,16).
	fn set_block_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropBlockSize_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_block_size")
	}
	
	/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
	fn block_stride(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_getPropBlockStride_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: block_stride")
	}
	
	/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
	fn set_block_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropBlockStride_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_block_stride")
	}
	
	/// Cell size. Default value is Size(8,8).
	fn cell_size(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_getPropCellSize_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: cell_size")
	}
	
	/// Cell size. Default value is Size(8,8).
	fn set_cell_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropCellSize_Size(self.as_raw_mut_HOGDescriptor(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_cell_size")
	}
	
	/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
	fn nbins(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_getPropNbins_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: nbins")
	}
	
	/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
	fn set_nbins(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropNbins_int(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_nbins")
	}
	
	/// not documented
	fn deriv_aperture(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_getPropDerivAperture_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: deriv_aperture")
	}
	
	/// not documented
	fn set_deriv_aperture(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropDerivAperture_int(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_deriv_aperture")
	}
	
	/// Gaussian smoothing window parameter.
	fn win_sigma(&self) -> f64 {
		unsafe { sys::cv_HOGDescriptor_getPropWinSigma_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: win_sigma")
	}
	
	/// Gaussian smoothing window parameter.
	fn set_win_sigma(&mut self, val: f64) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropWinSigma_double(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_win_sigma")
	}
	
	/// histogramNormType
	fn histogram_norm_type(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_getPropHistogramNormType_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: histogram_norm_type")
	}
	
	/// histogramNormType
	fn set_histogram_norm_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropHistogramNormType_int(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_histogram_norm_type")
	}
	
	/// L2-Hys normalization method shrinkage.
	fn l2_hys_threshold(&self) -> f64 {
		unsafe { sys::cv_HOGDescriptor_getPropL2HysThreshold_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: l2_hys_threshold")
	}
	
	/// L2-Hys normalization method shrinkage.
	fn set_l2_hys_threshold(&mut self, val: f64) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropL2HysThreshold_double(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_l2_hys_threshold")
	}
	
	/// Flag to specify whether the gamma correction preprocessing is required or not.
	fn gamma_correction(&self) -> bool {
		unsafe { sys::cv_HOGDescriptor_getPropGammaCorrection_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: gamma_correction")
	}
	
	/// Flag to specify whether the gamma correction preprocessing is required or not.
	fn set_gamma_correction(&mut self, val: bool) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropGammaCorrection_bool(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_gamma_correction")
	}
	
	/// coefficients for the linear SVM classifier.
	fn svm_detector(&mut self) -> core::Vector::<f32> {
		unsafe { sys::cv_HOGDescriptor_getPropSvmDetector(self.as_raw_mut_HOGDescriptor()) }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } ).expect("Infallible function failed: svm_detector")
	}
	
	/// coefficients for the linear SVM classifier.
	fn set_svm_detector_vec(&mut self, mut val: core::Vector::<f32>) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropSvmDetector_vector_float_(self.as_raw_mut_HOGDescriptor(), val.as_raw_mut_VectorOff32()) }.into_result().expect("Infallible function failed: set_svm_detector_vec")
	}
	
	/// coefficients for the linear SVM classifier used when OpenCL is enabled
	fn ocl_svm_detector(&mut self) -> core::UMat {
		unsafe { sys::cv_HOGDescriptor_getPropOclSvmDetector(self.as_raw_mut_HOGDescriptor()) }.into_result().map(|r| unsafe { core::UMat::opencv_from_extern(r) } ).expect("Infallible function failed: ocl_svm_detector")
	}
	
	/// coefficients for the linear SVM classifier used when OpenCL is enabled
	fn set_ocl_svm_detector(&mut self, mut val: core::UMat) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropOclSvmDetector_UMat(self.as_raw_mut_HOGDescriptor(), val.as_raw_mut_UMat()) }.into_result().expect("Infallible function failed: set_ocl_svm_detector")
	}
	
	/// not documented
	fn free_coef(&self) -> f32 {
		unsafe { sys::cv_HOGDescriptor_getPropFree_coef_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: free_coef")
	}
	
	/// not documented
	fn set_free_coef(&mut self, val: f32) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropFree_coef_float(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_free_coef")
	}
	
	/// Maximum number of detection window increases. Default value is 64
	fn nlevels(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_getPropNlevels_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: nlevels")
	}
	
	/// Maximum number of detection window increases. Default value is 64
	fn set_nlevels(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropNlevels_int(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_nlevels")
	}
	
	/// Indicates signed gradient will be used or not
	fn signed_gradient(&self) -> bool {
		unsafe { sys::cv_HOGDescriptor_getPropSignedGradient_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: signed_gradient")
	}
	
	/// Indicates signed gradient will be used or not
	fn set_signed_gradient(&mut self, val: bool) -> () {
		unsafe { sys::cv_HOGDescriptor_setPropSignedGradient_bool(self.as_raw_mut_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_signed_gradient")
	}
	
	/// Returns the number of coefficients required for the classification.
	fn get_descriptor_size(&self) -> Result<size_t> {
		unsafe { sys::cv_HOGDescriptor_getDescriptorSize_const(self.as_raw_HOGDescriptor()) }.into_result()
	}
	
	/// Checks if detector size equal to descriptor size.
	fn check_detector_size(&self) -> Result<bool> {
		unsafe { sys::cv_HOGDescriptor_checkDetectorSize_const(self.as_raw_HOGDescriptor()) }.into_result()
	}
	
	/// Returns winSigma value
	fn get_win_sigma(&self) -> Result<f64> {
		unsafe { sys::cv_HOGDescriptor_getWinSigma_const(self.as_raw_HOGDescriptor()) }.into_result()
	}
	
	/// @example samples/cpp/peopledetect.cpp
	/// /
	/// Sets coefficients for the linear SVM classifier.
	/// ## Parameters
	/// * _svmdetector: coefficients for the linear SVM classifier.
	fn set_svm_detector(&mut self, _svmdetector: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(_svmdetector);
		unsafe { sys::cv_HOGDescriptor_setSVMDetector_const__InputArrayR(self.as_raw_mut_HOGDescriptor(), _svmdetector.as_raw__InputArray()) }.into_result()
	}
	
	/// Reads HOGDescriptor parameters from a file node.
	/// ## Parameters
	/// * fn: File node
	fn read(&mut self, fn_: &mut core::FileNode) -> Result<bool> {
		unsafe { sys::cv_HOGDescriptor_read_FileNodeR(self.as_raw_mut_HOGDescriptor(), fn_.as_raw_mut_FileNode()) }.into_result()
	}
	
	/// Stores HOGDescriptor parameters in a file storage.
	/// ## Parameters
	/// * fs: File storage
	/// * objname: Object name
	fn write(&self, fs: &mut core::FileStorage, objname: &str) -> Result<()> {
		extern_container_arg!(objname);
		unsafe { sys::cv_HOGDescriptor_write_const_FileStorageR_const_StringR(self.as_raw_HOGDescriptor(), fs.as_raw_mut_FileStorage(), objname.opencv_as_extern()) }.into_result()
	}
	
	/// loads coefficients for the linear SVM classifier from a file
	/// ## Parameters
	/// * filename: Name of the file to read.
	/// * objname: The optional name of the node to read (if empty, the first top-level node will be used).
	/// 
	/// ## C++ default parameters
	/// * objname: String()
	fn load(&mut self, filename: &str, objname: &str) -> Result<bool> {
		extern_container_arg!(filename);
		extern_container_arg!(objname);
		unsafe { sys::cv_HOGDescriptor_load_const_StringR_const_StringR(self.as_raw_mut_HOGDescriptor(), filename.opencv_as_extern(), objname.opencv_as_extern()) }.into_result()
	}
	
	/// saves coefficients for the linear SVM classifier to a file
	/// ## Parameters
	/// * filename: File name
	/// * objname: Object name
	/// 
	/// ## C++ default parameters
	/// * objname: String()
	fn save(&self, filename: &str, objname: &str) -> Result<()> {
		extern_container_arg!(filename);
		extern_container_arg!(objname);
		unsafe { sys::cv_HOGDescriptor_save_const_const_StringR_const_StringR(self.as_raw_HOGDescriptor(), filename.opencv_as_extern(), objname.opencv_as_extern()) }.into_result()
	}
	
	/// clones the HOGDescriptor
	/// ## Parameters
	/// * c: cloned HOGDescriptor
	fn copy_to(&self, c: &mut crate::objdetect::HOGDescriptor) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_copyTo_const_HOGDescriptorR(self.as_raw_HOGDescriptor(), c.as_raw_mut_HOGDescriptor()) }.into_result()
	}
	
	/// @example samples/cpp/train_HOG.cpp
	/// /
	/// Computes HOG descriptors of given image.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U containing an image where HOG features will be calculated.
	/// * descriptors: Matrix of the type CV_32F
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * locations: Vector of Point
	/// 
	/// ## C++ default parameters
	/// * win_stride: Size()
	/// * padding: Size()
	/// * locations: std::vector<Point>()
	fn compute(&self, img: &dyn core::ToInputArray, descriptors: &mut core::Vector::<f32>, win_stride: core::Size, padding: core::Size, locations: &core::Vector::<core::Point>) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_HOGDescriptor_compute_const_const__InputArrayR_vector_float_R_Size_Size_const_vector_Point_R(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), descriptors.as_raw_mut_VectorOff32(), win_stride.opencv_as_extern(), padding.opencv_as_extern(), locations.as_raw_VectorOfPoint()) }.into_result()
	}
	
	/// Performs object detection without a multi-scale window.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
	/// * weights: Vector that will contain confidence values for each detected object.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * searchLocations: Vector of Point includes set of requested locations to be evaluated.
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * search_locations: std::vector<Point>()
	fn detect_weights(&self, img: &core::Mat, found_locations: &mut core::Vector::<core::Point>, weights: &mut core::Vector::<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &core::Vector::<core::Point>) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatR_vector_Point_R_vector_double_R_double_Size_Size_const_vector_Point_R(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfPoint(), weights.as_raw_mut_VectorOff64(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), search_locations.as_raw_VectorOfPoint()) }.into_result()
	}
	
	/// Performs object detection without a multi-scale window.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * searchLocations: Vector of Point includes locations to search.
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * search_locations: std::vector<Point>()
	fn detect(&self, img: &core::Mat, found_locations: &mut core::Vector::<core::Point>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &core::Vector::<core::Point>) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatR_vector_Point_R_double_Size_Size_const_vector_Point_R(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfPoint(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), search_locations.as_raw_VectorOfPoint()) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
	/// * foundWeights: Vector that will contain confidence values for each detected object.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * scale: Coefficient of the detection window increase.
	/// * finalThreshold: Final threshold
	/// * useMeanshiftGrouping: indicates grouping algorithm
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * scale: 1.05
	/// * final_threshold: 2.0
	/// * use_meanshift_grouping: false
	fn detect_multi_scale_weights(&self, img: &dyn core::ToInputArray, found_locations: &mut core::Vector::<core::Rect>, found_weights: &mut core::Vector::<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_vector_double_R_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), found_weights.as_raw_mut_VectorOff64(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), scale, final_threshold, use_meanshift_grouping) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * scale: Coefficient of the detection window increase.
	/// * finalThreshold: Final threshold
	/// * useMeanshiftGrouping: indicates grouping algorithm
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * scale: 1.05
	/// * final_threshold: 2.0
	/// * use_meanshift_grouping: false
	fn detect_multi_scale(&self, img: &dyn core::ToInputArray, found_locations: &mut core::Vector::<core::Rect>, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_mut_VectorOfRect(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern(), scale, final_threshold, use_meanshift_grouping) }.into_result()
	}
	
	///  Computes gradients and quantized gradient orientations.
	/// ## Parameters
	/// * img: Matrix contains the image to be computed
	/// * grad: Matrix of type CV_32FC2 contains computed gradients
	/// * angleOfs: Matrix of type CV_8UC2 contains quantized gradient orientations
	/// * paddingTL: Padding from top-left
	/// * paddingBR: Padding from bottom-right
	/// 
	/// ## C++ default parameters
	/// * padding_tl: Size()
	/// * padding_br: Size()
	fn compute_gradient(&self, img: &core::Mat, grad: &mut core::Mat, angle_ofs: &mut core::Mat, padding_tl: core::Size, padding_br: core::Size) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_computeGradient_const_const_MatR_MatR_MatR_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), grad.as_raw_mut_Mat(), angle_ofs.as_raw_mut_Mat(), padding_tl.opencv_as_extern(), padding_br.opencv_as_extern()) }.into_result()
	}
	
	/// evaluate specified ROI and return confidence value for each location
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * locations: Vector of Point
	/// * foundLocations: Vector of Point where each Point is detected object's top-left point.
	/// * confidences: confidences
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually
	/// it is 0 and should be specified in the detector coefficients (as the last free coefficient). But if
	/// the free coefficient is omitted (which is allowed), you can specify it manually here
	/// * winStride: winStride
	/// * padding: padding
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	fn detect_roi(&self, img: &core::Mat, locations: &core::Vector::<core::Point>, found_locations: &mut core::Vector::<core::Point>, confidences: &mut core::Vector::<f64>, hit_threshold: f64, win_stride: core::Size, padding: core::Size) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detectROI_const_const_MatR_const_vector_Point_R_vector_Point_R_vector_double_R_double_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), locations.as_raw_VectorOfPoint(), found_locations.as_raw_mut_VectorOfPoint(), confidences.as_raw_mut_VectorOff64(), hit_threshold, win_stride.opencv_as_extern(), padding.opencv_as_extern()) }.into_result()
	}
	
	/// evaluate specified ROI and return confidence value for each location in multiple scales
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
	/// * locations: Vector of DetectionROI
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually it is 0 and should be specified
	/// in the detector coefficients (as the last free coefficient). But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * group_threshold: 0
	fn detect_multi_scale_roi(&self, img: &core::Mat, found_locations: &mut core::Vector::<core::Rect>, locations: &mut core::Vector::<crate::objdetect::DetectionROI>, hit_threshold: f64, group_threshold: i32) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detectMultiScaleROI_const_const_MatR_vector_Rect_R_vector_DetectionROI_R_double_int(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_mut_VectorOfRect(), locations.as_raw_mut_VectorOfDetectionROI(), hit_threshold, group_threshold) }.into_result()
	}
	
	/// read/parse Dalal's alt model file
	/// ## Parameters
	/// * modelfile: Path of Dalal's alt model file.
	fn read_alt_model(&mut self, modelfile: &str) -> Result<()> {
		extern_container_arg!(mut modelfile);
		unsafe { sys::cv_HOGDescriptor_readALTModel_String(self.as_raw_mut_HOGDescriptor(), modelfile.opencv_as_extern_mut()) }.into_result()
	}
	
	/// Groups the object candidate rectangles.
	/// ## Parameters
	/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped rectangles. (The Python list is not modified in place.)
	/// * weights: Input/output vector of weights of rectangles. Output vector includes weights of retained and grouped rectangles. (The Python list is not modified in place.)
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
	/// * eps: Relative difference between sides of the rectangles to merge them into a group.
	fn group_rectangles(&self, rect_list: &mut core::Vector::<core::Rect>, weights: &mut core::Vector::<f64>, group_threshold: i32, eps: f64) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_groupRectangles_const_vector_Rect_R_vector_double_R_int_double(self.as_raw_HOGDescriptor(), rect_list.as_raw_mut_VectorOfRect(), weights.as_raw_mut_VectorOff64(), group_threshold, eps) }.into_result()
	}
	
}

/// Implementation of HOG (Histogram of Oriented Gradients) descriptor and object detector.
/// 
/// the HOG descriptor algorithm introduced by Navneet Dalal and Bill Triggs [Dalal2005](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Dalal2005) .
/// 
/// useful links:
/// 
/// https://hal.inria.fr/inria-00548512/document/
/// 
/// https://en.wikipedia.org/wiki/Histogram_of_oriented_gradients
/// 
/// https://software.intel.com/en-us/ipp-dev-reference-histogram-of-oriented-gradients-hog-descriptor
/// 
/// http://www.learnopencv.com/histogram-of-oriented-gradients
/// 
/// http://www.learnopencv.com/handwritten-digits-classification-an-opencv-c-python-tutorial
pub struct HOGDescriptor {
	ptr: *mut c_void
}

opencv_type_boxed! { HOGDescriptor }

impl Drop for HOGDescriptor {
	fn drop(&mut self) {
		extern "C" { fn cv_HOGDescriptor_delete(instance: *mut c_void); }
		unsafe { cv_HOGDescriptor_delete(self.as_raw_mut_HOGDescriptor()) };
	}
}

impl HOGDescriptor {
	#[inline] pub fn as_raw_HOGDescriptor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_HOGDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for HOGDescriptor {}

impl crate::objdetect::HOGDescriptorTrait for HOGDescriptor {
	#[inline] fn as_raw_HOGDescriptor(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_HOGDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HOGDescriptor {
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	pub fn default() -> Result<crate::objdetect::HOGDescriptor> {
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor() }.into_result().map(|r| unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(r) } )
	}
	
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * _winSize: sets winSize with given value.
	/// * _blockSize: sets blockSize with given value.
	/// * _blockStride: sets blockStride with given value.
	/// * _cellSize: sets cellSize with given value.
	/// * _nbins: sets nbins with given value.
	/// * _derivAperture: sets derivAperture with given value.
	/// * _winSigma: sets winSigma with given value.
	/// * _histogramNormType: sets histogramNormType with given value.
	/// * _L2HysThreshold: sets L2HysThreshold with given value.
	/// * _gammaCorrection: sets gammaCorrection with given value.
	/// * _nlevels: sets nlevels with given value.
	/// * _signedGradient: sets signedGradient with given value.
	/// 
	/// ## C++ default parameters
	/// * _deriv_aperture: 1
	/// * _win_sigma: -1
	/// * _histogram_norm_type: HOGDescriptor::L2Hys
	/// * _l2_hys_threshold: 0.2
	/// * _gamma_correction: false
	/// * _nlevels: HOGDescriptor::DEFAULT_NLEVELS
	/// * _signed_gradient: false
	pub fn new(_win_size: core::Size, _block_size: core::Size, _block_stride: core::Size, _cell_size: core::Size, _nbins: i32, _deriv_aperture: i32, _win_sigma: f64, _histogram_norm_type: i32, _l2_hys_threshold: f64, _gamma_correction: bool, _nlevels: i32, _signed_gradient: bool) -> Result<crate::objdetect::HOGDescriptor> {
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_int_double_bool_int_bool(_win_size.opencv_as_extern(), _block_size.opencv_as_extern(), _block_stride.opencv_as_extern(), _cell_size.opencv_as_extern(), _nbins, _deriv_aperture, _win_sigma, _histogram_norm_type, _l2_hys_threshold, _gamma_correction, _nlevels, _signed_gradient) }.into_result().map(|r| unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(r) } )
	}
	
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * filename: the file name containing  HOGDescriptor properties and coefficients of the trained classifier
	pub fn new_from_file(filename: &str) -> Result<crate::objdetect::HOGDescriptor> {
		extern_container_arg!(filename);
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_const_StringR(filename.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(r) } )
	}
	
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * d: the HOGDescriptor which cloned to create a new one.
	pub fn copy(d: &crate::objdetect::HOGDescriptor) -> Result<crate::objdetect::HOGDescriptor> {
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(d.as_raw_HOGDescriptor()) }.into_result().map(|r| unsafe { crate::objdetect::HOGDescriptor::opencv_from_extern(r) } )
	}
	
	/// Returns coefficients of the classifier trained for people detection (for 64x128 windows).
	pub fn get_default_people_detector() -> Result<core::Vector::<f32>> {
		unsafe { sys::cv_HOGDescriptor_getDefaultPeopleDetector() }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } )
	}
	
	/// @example samples/tapi/hog.cpp
	/// /
	/// Returns coefficients of the classifier trained for people detection (for 48x96 windows).
	pub fn get_daimler_people_detector() -> Result<core::Vector::<f32>> {
		unsafe { sys::cv_HOGDescriptor_getDaimlerPeopleDetector() }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } )
	}
	
}

pub trait QRCodeDetectorTrait {
	fn as_raw_QRCodeDetector(&self) -> *const c_void;
	fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void;

	/// sets the epsilon used during the horizontal scan of QR code stop marker detection.
	/// ## Parameters
	/// * epsX: Epsilon neighborhood, which allows you to determine the horizontal pattern
	/// of the scheme 1:1:3:1:1 according to QR code standard.
	fn set_eps_x(&mut self, eps_x: f64) -> Result<()> {
		unsafe { sys::cv_QRCodeDetector_setEpsX_double(self.as_raw_mut_QRCodeDetector(), eps_x) }.into_result()
	}
	
	/// sets the epsilon used during the vertical scan of QR code stop marker detection.
	/// ## Parameters
	/// * epsY: Epsilon neighborhood, which allows you to determine the vertical pattern
	/// of the scheme 1:1:3:1:1 according to QR code standard.
	fn set_eps_y(&mut self, eps_y: f64) -> Result<()> {
		unsafe { sys::cv_QRCodeDetector_setEpsY_double(self.as_raw_mut_QRCodeDetector(), eps_y) }.into_result()
	}
	
	/// Detects QR code in image and returns the quadrangle containing the code.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing (or not) QR code.
	/// * points: Output vector of vertices of the minimum-area quadrangle containing the code.
	fn detect(&self, img: &dyn core::ToInputArray, points: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		unsafe { sys::cv_QRCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray()) }.into_result()
	}
	
	/// Decodes QR code in image once it's found by the detect() method.
	/// 
	/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	/// 
	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	fn decode(&mut self, img: &dyn core::ToInputArray, points: &dyn core::ToInputArray, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<String> {
		input_array_arg!(img);
		input_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), straight_qrcode.as_raw__OutputArray()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Both detects and decodes QR code
	/// 
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: optional output array of vertices of the found QR code quadrangle. Will be empty if not found.
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	/// 
	/// ## C++ default parameters
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	fn detect_and_decode(&mut self, img: &dyn core::ToInputArray, points: &mut dyn core::ToOutputArray, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<String> {
		input_array_arg!(img);
		output_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_detectAndDecode_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Detects QR codes in image and returns the vector of the quadrangles containing the codes.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing (or not) QR codes.
	/// * points: Output vector of vector of vertices of the minimum-area quadrangle containing the codes.
	fn detect_multi(&self, img: &dyn core::ToInputArray, points: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		unsafe { sys::cv_QRCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray()) }.into_result()
	}
	
	/// Decodes QR codes in image once it's found by the detect() method.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR codes.
	/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
	/// * points: vector of Quadrangle vertices found by detect() method (or some other algorithm).
	/// * straight_qrcode: The optional output vector of images containing rectified and binarized QR codes
	/// 
	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	fn decode_multi(&self, img: &dyn core::ToInputArray, points: &dyn core::ToInputArray, decoded_info: &mut core::Vector::<String>, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		input_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vector_String_R_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), straight_qrcode.as_raw__OutputArray()) }.into_result()
	}
	
	/// Both detects and decodes QR codes
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR codes.
	/// * decoded_info: UTF8-encoded output vector of string or empty vector of string if the codes cannot be decoded.
	/// * points: optional output vector of vertices of the found QR code quadrangles. Will be empty if not found.
	/// * straight_qrcode: The optional output vector of images containing rectified and binarized QR codes
	/// 
	/// ## C++ default parameters
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	fn detect_and_decode_multi(&self, img: &dyn core::ToInputArray, decoded_info: &mut core::Vector::<String>, points: &mut dyn core::ToOutputArray, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vector_String_R_const__OutputArrayR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	fn decode_multi_1(&self, img: &dyn core::ToInputArray, points: &dyn core::ToInputArray, decoded_info: &mut core::Vector::<String>, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		input_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vector_string_R_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), straight_qrcode.as_raw__OutputArray()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	fn detect_and_decode_multi_1(&self, img: &dyn core::ToInputArray, decoded_info: &mut core::Vector::<String>, points: &mut dyn core::ToOutputArray, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vector_string_R_const__OutputArrayR_const__OutputArrayR(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), decoded_info.as_raw_mut_VectorOfString(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray()) }.into_result()
	}
	
}

pub struct QRCodeDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { QRCodeDetector }

impl Drop for QRCodeDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_QRCodeDetector_delete(instance: *mut c_void); }
		unsafe { cv_QRCodeDetector_delete(self.as_raw_mut_QRCodeDetector()) };
	}
}

impl QRCodeDetector {
	#[inline] pub fn as_raw_QRCodeDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for QRCodeDetector {}

impl crate::objdetect::QRCodeDetectorTrait for QRCodeDetector {
	#[inline] fn as_raw_QRCodeDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_QRCodeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QRCodeDetector {
	pub fn default() -> Result<crate::objdetect::QRCodeDetector> {
		unsafe { sys::cv_QRCodeDetector_QRCodeDetector() }.into_result().map(|r| unsafe { crate::objdetect::QRCodeDetector::opencv_from_extern(r) } )
	}
	
}

/// class for grouping object candidates, detected by Cascade Classifier, HOG etc.
/// instance of the class is to be passed to cv::partition (see cxoperations.hpp)
pub trait SimilarRectsTrait {
	fn as_raw_SimilarRects(&self) -> *const c_void;
	fn as_raw_mut_SimilarRects(&mut self) -> *mut c_void;

	fn eps(&self) -> f64 {
		unsafe { sys::cv_SimilarRects_getPropEps_const(self.as_raw_SimilarRects()) }.into_result().expect("Infallible function failed: eps")
	}
	
	fn set_eps(&mut self, val: f64) -> () {
		unsafe { sys::cv_SimilarRects_setPropEps_double(self.as_raw_mut_SimilarRects(), val) }.into_result().expect("Infallible function failed: set_eps")
	}
	
}

/// class for grouping object candidates, detected by Cascade Classifier, HOG etc.
/// instance of the class is to be passed to cv::partition (see cxoperations.hpp)
pub struct SimilarRects {
	ptr: *mut c_void
}

opencv_type_boxed! { SimilarRects }

impl Drop for SimilarRects {
	fn drop(&mut self) {
		extern "C" { fn cv_SimilarRects_delete(instance: *mut c_void); }
		unsafe { cv_SimilarRects_delete(self.as_raw_mut_SimilarRects()) };
	}
}

impl SimilarRects {
	#[inline] pub fn as_raw_SimilarRects(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SimilarRects(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SimilarRects {}

impl crate::objdetect::SimilarRectsTrait for SimilarRects {
	#[inline] fn as_raw_SimilarRects(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SimilarRects(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SimilarRects {
	pub fn new(_eps: f64) -> Result<crate::objdetect::SimilarRects> {
		unsafe { sys::cv_SimilarRects_SimilarRects_double(_eps) }.into_result().map(|r| unsafe { crate::objdetect::SimilarRects::opencv_from_extern(r) } )
	}
	
}

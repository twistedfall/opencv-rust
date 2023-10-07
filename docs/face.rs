pub mod face {
	//! # Face Analysis
	//! 
	//! - [face_changelog]
	//! - [tutorial_face_main]
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::PredictCollectorTraitConst, super::PredictCollectorTrait, super::StandardCollectorTraitConst, super::StandardCollectorTrait, super::FaceRecognizerTraitConst, super::FaceRecognizerTrait, super::BasicFaceRecognizerTraitConst, super::BasicFaceRecognizerTrait, super::EigenFaceRecognizerTraitConst, super::EigenFaceRecognizerTrait, super::FisherFaceRecognizerTraitConst, super::FisherFaceRecognizerTrait, super::LBPHFaceRecognizerTraitConst, super::LBPHFaceRecognizerTrait, super::FacemarkTraitConst, super::FacemarkTrait, super::CParamsTraitConst, super::CParamsTrait, super::FacemarkTrainTraitConst, super::FacemarkTrainTrait, super::FacemarkLBF_ParamsTraitConst, super::FacemarkLBF_ParamsTrait, super::FacemarkLBFTraitConst, super::FacemarkLBFTrait, super::FacemarkAAM_ParamsTraitConst, super::FacemarkAAM_ParamsTrait, super::FacemarkAAM_ConfigTraitConst, super::FacemarkAAM_ConfigTrait, super::FacemarkAAM_DataTraitConst, super::FacemarkAAM_DataTrait, super::FacemarkAAM_Model_TextureTraitConst, super::FacemarkAAM_Model_TextureTrait, super::FacemarkAAM_ModelTraitConst, super::FacemarkAAM_ModelTrait, super::FacemarkAAMTraitConst, super::FacemarkAAMTrait, super::FacemarkKazemi_ParamsTraitConst, super::FacemarkKazemi_ParamsTrait, super::FacemarkKazemiTraitConst, super::FacemarkKazemiTrait, super::MACETraitConst, super::MACETrait, super::BIFTraitConst, super::BIFTrait };
	}
	
	pub type FN_FaceDetector = Option<Box<dyn FnMut(*const c_void, *const c_void) -> bool + Send + Sync + 'static>>;
	/// construct an AAM facemark detector
	#[inline]
	pub fn create_facemark_aam() -> Result<core::Ptr<crate::face::Facemark>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_createFacemarkAAM(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::face::Facemark>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// construct a Kazemi facemark detector
	#[inline]
	pub fn create_facemark_kazemi() -> Result<core::Ptr<crate::face::Facemark>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_createFacemarkKazemi(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::face::Facemark>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// construct an LBF facemark detector
	#[inline]
	pub fn create_facemark_lbf() -> Result<core::Ptr<crate::face::Facemark>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_createFacemarkLBF(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::face::Facemark>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Utility to draw the detected facial landmark points
	/// 
	/// ## Parameters
	/// * image: The input image to be processed.
	/// * points: Contains the data of points which will be drawn.
	/// * color: The color of points in BGR format represented by cv::Scalar.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// std::vector<Rect> faces;
	/// std::vector<std::vector<Point2f> > landmarks;
	/// facemark->getFaces(img, faces);
	/// facemark->fit(img, faces, landmarks);
	/// for(int j=0;j<rects.size();j++){
	///    face::drawFacemarks(frame, landmarks[j], Scalar(0,0,255));
	/// }
	/// ```
	/// 
	/// 
	/// ## Note
	/// This alternative version of [draw_facemarks] function uses the following default values for its arguments:
	/// * color: Scalar(255,0,0)
	#[inline]
	pub fn draw_facemarks_def(image: &mut impl core::ToInputOutputArray, points: &impl core::ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR(image.as_raw__InputOutputArray(), points.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Utility to draw the detected facial landmark points
	/// 
	/// ## Parameters
	/// * image: The input image to be processed.
	/// * points: Contains the data of points which will be drawn.
	/// * color: The color of points in BGR format represented by cv::Scalar.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// std::vector<Rect> faces;
	/// std::vector<std::vector<Point2f> > landmarks;
	/// facemark->getFaces(img, faces);
	/// facemark->fit(img, faces, landmarks);
	/// for(int j=0;j<rects.size();j++){
	///    face::drawFacemarks(frame, landmarks[j], Scalar(0,0,255));
	/// }
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * color: Scalar(255,0,0)
	#[inline]
	pub fn draw_facemarks(image: &mut impl core::ToInputOutputArray, points: &impl core::ToInputArray, color: core::Scalar) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), points.as_raw__InputArray(), color.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_faces_haar(image: &impl core::ToInputArray, faces: &mut impl core::ToOutputArray, face_cascade_name: &str) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(faces);
		extern_container_arg!(face_cascade_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_getFacesHAAR_const__InputArrayR_const__OutputArrayR_const_StringR(image.as_raw__InputArray(), faces.as_raw__OutputArray(), face_cascade_name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Default face detector
	/// This function is mainly utilized by the implementation of a Facemark Algorithm.
	/// End users are advised to use function Facemark::getFaces which can be manually defined
	/// and circumvented to the algorithm by Facemark::setFaceDetector.
	/// 
	/// ## Parameters
	/// * image: The input image to be processed.
	/// * faces: Output of the function which represent region of interest of the detected faces.
	/// Each face is stored in cv::Rect container.
	/// * params: detector parameters
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// std::vector<cv::Rect> faces;
	/// CParams params("haarcascade_frontalface_alt.xml");
	/// cv::face::getFaces(frame, faces, &params);
	/// for(int j=0;j<faces.size();j++){
	///    cv::rectangle(frame, faces[j], cv::Scalar(255,0,255));
	/// }
	/// cv::imshow("detection", frame);
	/// ```
	/// 
	#[inline]
	pub fn get_faces(image: &impl core::ToInputArray, faces: &mut impl core::ToOutputArray, params: &mut crate::face::CParams) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(faces);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_getFaces_const__InputArrayR_const__OutputArrayR_CParamsX(image.as_raw__InputArray(), faces.as_raw__OutputArray(), params.as_raw_mut_CParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// A utility to load list of paths to training image and annotation file.
	/// ## Parameters
	/// * imageList: The specified file contains paths to the training images.
	/// * annotationList: The specified file contains paths to the training annotations.
	/// * images: The loaded paths of training images.
	/// * annotations: The loaded paths of annotation files.
	/// 
	/// Example of usage:
	/// ```C++
	/// String imageFiles = "images_path.txt";
	/// String ptsFiles = "annotations_path.txt";
	/// std::vector<String> images_train;
	/// std::vector<String> landmarks_train;
	/// loadDatasetList(imageFiles,ptsFiles,images_train,landmarks_train);
	/// ```
	/// 
	#[inline]
	pub fn load_dataset_list(image_list: &str, annotation_list: &str, images: &mut core::Vector<String>, annotations: &mut core::Vector<String>) -> Result<bool> {
		extern_container_arg!(mut image_list);
		extern_container_arg!(mut annotation_list);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadDatasetList_String_String_vectorLStringGR_vectorLStringGR(image_list.opencv_as_extern_mut(), annotation_list.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), annotations.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// A utility to load facial landmark information from a given file.
	/// 
	/// ## Parameters
	/// * filename: The filename of file contains the facial landmarks data.
	/// * points: The loaded facial landmark points.
	/// * offset: An offset value to adjust the loaded points.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// std::vector<Point2f> points;
	/// face::loadFacePoints("filename.txt", points, 0.0f);
	/// ```
	/// 
	/// 
	/// The annotation file should follow the default format which is
	/// ```C++
	/// version: 1
	/// n_points:  68
	/// {
	/// 212.716603 499.771793
	/// 230.232816 566.290071
	/// ...
	/// }
	/// ```
	/// 
	/// where n_points is the number of points considered
	/// and each point is represented as its position in x and y.
	/// 
	/// ## Note
	/// This alternative version of [load_face_points] function uses the following default values for its arguments:
	/// * offset: 0.0f
	#[inline]
	pub fn load_face_points_def(filename: &str, points: &mut impl core::ToOutputArray) -> Result<bool> {
		extern_container_arg!(mut filename);
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadFacePoints_String_const__OutputArrayR(filename.opencv_as_extern_mut(), points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// A utility to load facial landmark information from a given file.
	/// 
	/// ## Parameters
	/// * filename: The filename of file contains the facial landmarks data.
	/// * points: The loaded facial landmark points.
	/// * offset: An offset value to adjust the loaded points.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// std::vector<Point2f> points;
	/// face::loadFacePoints("filename.txt", points, 0.0f);
	/// ```
	/// 
	/// 
	/// The annotation file should follow the default format which is
	/// ```C++
	/// version: 1
	/// n_points:  68
	/// {
	/// 212.716603 499.771793
	/// 230.232816 566.290071
	/// ...
	/// }
	/// ```
	/// 
	/// where n_points is the number of points considered
	/// and each point is represented as its position in x and y.
	/// 
	/// ## C++ default parameters
	/// * offset: 0.0f
	#[inline]
	pub fn load_face_points(filename: &str, points: &mut impl core::ToOutputArray, offset: f32) -> Result<bool> {
		extern_container_arg!(mut filename);
		output_array_arg!(points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadFacePoints_String_const__OutputArrayR_float(filename.opencv_as_extern_mut(), points.as_raw__OutputArray(), offset, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// A utility to load facial landmark information from the dataset.
	/// 
	/// ## Parameters
	/// * imageList: A file contains the list of image filenames in the training dataset.
	/// * groundTruth: A file contains the list of filenames
	/// where the landmarks points information are stored.
	/// The content in each file should follow the standard format (see face::loadFacePoints).
	/// * images: A vector where each element represent the filename of image in the dataset.
	/// Images are not loaded by default to save the memory.
	/// * facePoints: The loaded landmark points for all training data.
	/// * offset: An offset value to adjust the loaded points.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// cv::String imageFiles = "../data/images_train.txt";
	/// cv::String ptsFiles = "../data/points_train.txt";
	/// std::vector<String> images;
	/// std::vector<std::vector<Point2f> > facePoints;
	/// loadTrainingData(imageFiles, ptsFiles, images, facePoints, 0.0f);
	/// ```
	/// 
	/// 
	/// example of content in the images_train.txt
	/// ```C++
	/// /home/user/ibug/image_003_1.jpg
	/// /home/user/ibug/image_004_1.jpg
	/// /home/user/ibug/image_005_1.jpg
	/// /home/user/ibug/image_006.jpg
	/// ```
	/// 
	/// 
	/// example of content in the points_train.txt
	/// ```C++
	/// /home/user/ibug/image_003_1.pts
	/// /home/user/ibug/image_004_1.pts
	/// /home/user/ibug/image_005_1.pts
	/// /home/user/ibug/image_006.pts
	/// ```
	/// 
	/// 
	/// ## Note
	/// This alternative version of [load_training_data_1] function uses the following default values for its arguments:
	/// * offset: 0.0f
	#[inline]
	pub fn load_training_data_1_def(image_list: &str, ground_truth: &str, images: &mut core::Vector<String>, face_points: &mut impl core::ToOutputArray) -> Result<bool> {
		extern_container_arg!(mut image_list);
		extern_container_arg!(mut ground_truth);
		output_array_arg!(face_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadTrainingData_String_String_vectorLStringGR_const__OutputArrayR(image_list.opencv_as_extern_mut(), ground_truth.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// A utility to load facial landmark information from the dataset.
	/// 
	/// ## Parameters
	/// * imageList: A file contains the list of image filenames in the training dataset.
	/// * groundTruth: A file contains the list of filenames
	/// where the landmarks points information are stored.
	/// The content in each file should follow the standard format (see face::loadFacePoints).
	/// * images: A vector where each element represent the filename of image in the dataset.
	/// Images are not loaded by default to save the memory.
	/// * facePoints: The loaded landmark points for all training data.
	/// * offset: An offset value to adjust the loaded points.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// cv::String imageFiles = "../data/images_train.txt";
	/// cv::String ptsFiles = "../data/points_train.txt";
	/// std::vector<String> images;
	/// std::vector<std::vector<Point2f> > facePoints;
	/// loadTrainingData(imageFiles, ptsFiles, images, facePoints, 0.0f);
	/// ```
	/// 
	/// 
	/// example of content in the images_train.txt
	/// ```C++
	/// /home/user/ibug/image_003_1.jpg
	/// /home/user/ibug/image_004_1.jpg
	/// /home/user/ibug/image_005_1.jpg
	/// /home/user/ibug/image_006.jpg
	/// ```
	/// 
	/// 
	/// example of content in the points_train.txt
	/// ```C++
	/// /home/user/ibug/image_003_1.pts
	/// /home/user/ibug/image_004_1.pts
	/// /home/user/ibug/image_005_1.pts
	/// /home/user/ibug/image_006.pts
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * offset: 0.0f
	#[inline]
	pub fn load_training_data_1(image_list: &str, ground_truth: &str, images: &mut core::Vector<String>, face_points: &mut impl core::ToOutputArray, offset: f32) -> Result<bool> {
		extern_container_arg!(mut image_list);
		extern_container_arg!(mut ground_truth);
		output_array_arg!(face_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadTrainingData_String_String_vectorLStringGR_const__OutputArrayR_float(image_list.opencv_as_extern_mut(), ground_truth.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), offset, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// A utility to load facial landmark dataset from a single file.
	/// 
	/// ## Parameters
	/// * filename: The filename of a file that contains the dataset information.
	/// Each line contains the filename of an image followed by
	/// pairs of x and y values of facial landmarks points separated by a space.
	/// Example
	/// ```C++
	/// /home/user/ibug/image_003_1.jpg 336.820955 240.864510 334.238298 260.922709 335.266918 ...
	/// /home/user/ibug/image_005_1.jpg 376.158428 230.845712 376.736984 254.924635 383.265403 ...
	/// ```
	/// 
	/// * images: A vector where each element represent the filename of image in the dataset.
	/// Images are not loaded by default to save the memory.
	/// * facePoints: The loaded landmark points for all training data.
	/// * delim: Delimiter between each element, the default value is a whitespace.
	/// * offset: An offset value to adjust the loaded points.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// cv::String imageFiles = "../data/images_train.txt";
	/// cv::String ptsFiles = "../data/points_train.txt";
	/// std::vector<String> images;
	/// std::vector<std::vector<Point2f> > facePoints;
	/// loadTrainingData(imageFiles, ptsFiles, images, facePoints, 0.0f);
	/// ```
	/// 
	/// 
	/// ## Note
	/// This alternative version of [load_training_data] function uses the following default values for its arguments:
	/// * delim: ' '
	/// * offset: 0.0f
	#[inline]
	pub fn load_training_data_def(filename: &str, images: &mut core::Vector<String>, face_points: &mut impl core::ToOutputArray) -> Result<bool> {
		extern_container_arg!(mut filename);
		output_array_arg!(face_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadTrainingData_String_vectorLStringGR_const__OutputArrayR(filename.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// A utility to load facial landmark dataset from a single file.
	/// 
	/// ## Parameters
	/// * filename: The filename of a file that contains the dataset information.
	/// Each line contains the filename of an image followed by
	/// pairs of x and y values of facial landmarks points separated by a space.
	/// Example
	/// ```C++
	/// /home/user/ibug/image_003_1.jpg 336.820955 240.864510 334.238298 260.922709 335.266918 ...
	/// /home/user/ibug/image_005_1.jpg 376.158428 230.845712 376.736984 254.924635 383.265403 ...
	/// ```
	/// 
	/// * images: A vector where each element represent the filename of image in the dataset.
	/// Images are not loaded by default to save the memory.
	/// * facePoints: The loaded landmark points for all training data.
	/// * delim: Delimiter between each element, the default value is a whitespace.
	/// * offset: An offset value to adjust the loaded points.
	/// 
	/// <B>Example of usage</B>
	/// ```C++
	/// cv::String imageFiles = "../data/images_train.txt";
	/// cv::String ptsFiles = "../data/points_train.txt";
	/// std::vector<String> images;
	/// std::vector<std::vector<Point2f> > facePoints;
	/// loadTrainingData(imageFiles, ptsFiles, images, facePoints, 0.0f);
	/// ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * delim: ' '
	/// * offset: 0.0f
	#[inline]
	pub fn load_training_data(filename: &str, images: &mut core::Vector<String>, face_points: &mut impl core::ToOutputArray, delim: char, offset: f32) -> Result<bool> {
		extern_container_arg!(mut filename);
		output_array_arg!(face_points);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadTrainingData_String_vectorLStringGR_const__OutputArrayR_char_float(filename.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), u8::try_from(delim)? as c_char, offset, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This function extracts the data for training from .txt files which contains the corresponding image name and landmarks.
	/// The first file in each file should give the path of the image whose
	/// landmarks are being described in the file. Then in the subsequent
	/// lines there should be coordinates of the landmarks in the image
	/// i.e each line should be of the form x,y
	/// where x represents the x coordinate of the landmark and y represents
	/// the y coordinate of the landmark.
	/// 
	/// For reference you can see the files as provided in the
	/// <a href="http://www.ifp.illinois.edu/~vuongle2/helen/">HELEN dataset</a>
	/// 
	/// ## Parameters
	/// * filename: A vector of type cv::String containing name of the .txt files.
	/// * trainlandmarks: A vector of type cv::Point2f that would store shape or landmarks of all images.
	/// * trainimages: A vector of type cv::String which stores the name of images whose landmarks are tracked
	/// ## Returns
	/// A boolean value. It returns true when it reads the data successfully and false otherwise
	#[inline]
	pub fn load_training_data_2(mut filename: core::Vector<String>, trainlandmarks: &mut core::Vector<core::Vector<core::Point2f>>, trainimages: &mut core::Vector<String>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_face_loadTrainingData_vectorLStringG_vectorLvectorLPoint2fGGR_vectorLStringGR(filename.as_raw_mut_VectorOfString(), trainlandmarks.as_raw_mut_VectorOfVectorOfPoint2f(), trainimages.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::face::BIF]
	pub trait BIFTraitConst: core::AlgorithmTraitConst {
		fn as_raw_BIF(&self) -> *const c_void;
	
		/// ## Returns
		/// The number of filter bands used for computing BIF.
		#[inline]
		fn get_num_bands(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BIF_getNumBands_const(self.as_raw_BIF(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Returns
		/// The number of image rotations.
		#[inline]
		fn get_num_rotations(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BIF_getNumRotations_const(self.as_raw_BIF(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Computes features sby input image.
		/// ## Parameters
		/// * image: Input image (CV_32FC1).
		/// * features: Feature vector (CV_32FC1).
		#[inline]
		fn compute(&self, image: &impl core::ToInputArray, features: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(features);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BIF_compute_const_const__InputArrayR_const__OutputArrayR(self.as_raw_BIF(), image.as_raw__InputArray(), features.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::BIF]
	pub trait BIFTrait: core::AlgorithmTrait + crate::face::BIFTraitConst {
		fn as_raw_mut_BIF(&mut self) -> *mut c_void;
	
	}
	
	/// Implementation of bio-inspired features (BIF) from the paper:
	/// Guo, Guodong, et al. "Human age estimation using bio-inspired features."
	/// Computer Vision and Pattern Recognition, 2009. CVPR 2009.
	pub struct BIF {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BIF }
	
	impl Drop for BIF {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_BIF_delete(self.as_raw_mut_BIF()) };
		}
	}
	
	unsafe impl Send for BIF {}
	
	impl core::AlgorithmTraitConst for BIF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for BIF {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::BIFTraitConst for BIF {
		#[inline] fn as_raw_BIF(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::BIFTrait for BIF {
		#[inline] fn as_raw_mut_BIF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BIF {
		/// ## Parameters
		/// * num_bands: The number of filter bands (<=8) used for computing BIF.
		/// * num_rotations: The number of image rotations for computing BIF.
		/// ## Returns
		/// Object for computing BIF.
		/// 
		/// ## C++ default parameters
		/// * num_bands: 8
		/// * num_rotations: 12
		#[inline]
		pub fn create(num_bands: i32, num_rotations: i32) -> Result<core::Ptr<crate::face::BIF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BIF_create_int_int(num_bands, num_rotations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::BIF>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Parameters
		/// * num_bands: The number of filter bands (<=8) used for computing BIF.
		/// * num_rotations: The number of image rotations for computing BIF.
		/// ## Returns
		/// Object for computing BIF.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * num_bands: 8
		/// * num_rotations: 12
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::BIF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BIF_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::BIF>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { BIF, core::Algorithm, cv_face_BIF_to_Algorithm }
	
	impl std::fmt::Debug for BIF {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BIF")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::BasicFaceRecognizer]
	pub trait BasicFaceRecognizerTraitConst: crate::face::FaceRecognizerTraitConst {
		fn as_raw_BasicFaceRecognizer(&self) -> *const c_void;
	
		/// ## See also
		/// setNumComponents
		#[inline]
		fn get_num_components(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_getNumComponents_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setThreshold
		#[inline]
		fn get_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_getThreshold_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_projections(&self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_getProjections_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_labels(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_getLabels_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_eigen_values(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_getEigenValues_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_eigen_vectors(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_getEigenVectors_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_mean(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_getMean_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_write_const_FileStorageR(self.as_raw_BasicFaceRecognizer(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_empty_const(self.as_raw_BasicFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::BasicFaceRecognizer]
	pub trait BasicFaceRecognizerTrait: crate::face::BasicFaceRecognizerTraitConst + crate::face::FaceRecognizerTrait {
		fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void;
	
		/// ## See also
		/// setNumComponents getNumComponents
		#[inline]
		fn set_num_components(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_setNumComponents_int(self.as_raw_mut_BasicFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setThreshold getThreshold
		#[inline]
		fn set_threshold(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_setThreshold_double(self.as_raw_mut_BasicFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_BasicFaceRecognizer_read_const_FileNodeR(self.as_raw_mut_BasicFaceRecognizer(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct BasicFaceRecognizer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BasicFaceRecognizer }
	
	impl Drop for BasicFaceRecognizer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_BasicFaceRecognizer_delete(self.as_raw_mut_BasicFaceRecognizer()) };
		}
	}
	
	unsafe impl Send for BasicFaceRecognizer {}
	
	impl core::AlgorithmTraitConst for BasicFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for BasicFaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for BasicFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for BasicFaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizerTraitConst for BasicFaceRecognizer {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::BasicFaceRecognizerTrait for BasicFaceRecognizer {
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BasicFaceRecognizer {
	}
	
	boxed_cast_descendant! { BasicFaceRecognizer, crate::face::EigenFaceRecognizer, cv_face_BasicFaceRecognizer_to_EigenFaceRecognizer }
	
	boxed_cast_descendant! { BasicFaceRecognizer, crate::face::FisherFaceRecognizer, cv_face_BasicFaceRecognizer_to_FisherFaceRecognizer }
	
	boxed_cast_base! { BasicFaceRecognizer, core::Algorithm, cv_face_BasicFaceRecognizer_to_Algorithm }
	
	boxed_cast_base! { BasicFaceRecognizer, crate::face::FaceRecognizer, cv_face_BasicFaceRecognizer_to_FaceRecognizer }
	
	impl std::fmt::Debug for BasicFaceRecognizer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BasicFaceRecognizer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::CParams]
	pub trait CParamsTraitConst {
		fn as_raw_CParams(&self) -> *const c_void;
	
		/// the face detector
		#[inline]
		fn cascade(&self) -> String {
			let ret = unsafe { sys::cv_face_CParams_propCascade_const(self.as_raw_CParams()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// Parameter specifying how much the image size is reduced at each image scale.
		#[inline]
		fn scale_factor(&self) -> f64 {
			let ret = unsafe { sys::cv_face_CParams_propScaleFactor_const(self.as_raw_CParams()) };
			ret
		}
		
		/// Parameter specifying how many neighbors each candidate rectangle should have to retain it.
		#[inline]
		fn min_neighbors(&self) -> i32 {
			let ret = unsafe { sys::cv_face_CParams_propMinNeighbors_const(self.as_raw_CParams()) };
			ret
		}
		
		/// Minimum possible object size.
		#[inline]
		fn min_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_CParams_propMinSize_const(self.as_raw_CParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Maximum possible object size.
		#[inline]
		fn max_size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_CParams_propMaxSize_const(self.as_raw_CParams(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn face_cascade(&self) -> crate::objdetect::CascadeClassifier {
			let ret = unsafe { sys::cv_face_CParams_propFace_cascade_const(self.as_raw_CParams()) };
			let ret = unsafe { crate::objdetect::CascadeClassifier::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::face::CParams]
	pub trait CParamsTrait: crate::face::CParamsTraitConst {
		fn as_raw_mut_CParams(&mut self) -> *mut c_void;
	
		/// the face detector
		#[inline]
		fn set_cascade(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_face_CParams_propCascade_String(self.as_raw_mut_CParams(), val.opencv_as_extern_mut()) };
			ret
		}
		
		/// Parameter specifying how much the image size is reduced at each image scale.
		#[inline]
		fn set_scale_factor(&mut self, val: f64) {
			let ret = unsafe { sys::cv_face_CParams_propScaleFactor_double(self.as_raw_mut_CParams(), val) };
			ret
		}
		
		/// Parameter specifying how many neighbors each candidate rectangle should have to retain it.
		#[inline]
		fn set_min_neighbors(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_CParams_propMinNeighbors_int(self.as_raw_mut_CParams(), val) };
			ret
		}
		
		/// Minimum possible object size.
		#[inline]
		fn set_min_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_face_CParams_propMinSize_Size(self.as_raw_mut_CParams(), val.opencv_as_extern()) };
			ret
		}
		
		/// Maximum possible object size.
		#[inline]
		fn set_max_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_face_CParams_propMaxSize_Size(self.as_raw_mut_CParams(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_face_cascade(&mut self, mut val: crate::objdetect::CascadeClassifier) {
			let ret = unsafe { sys::cv_face_CParams_propFace_cascade_CascadeClassifier(self.as_raw_mut_CParams(), val.as_raw_mut_CascadeClassifier()) };
			ret
		}
		
	}
	
	pub struct CParams {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CParams }
	
	impl Drop for CParams {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_CParams_delete(self.as_raw_mut_CParams()) };
		}
	}
	
	unsafe impl Send for CParams {}
	
	impl crate::face::CParamsTraitConst for CParams {
		#[inline] fn as_raw_CParams(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::CParamsTrait for CParams {
		#[inline] fn as_raw_mut_CParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CParams {
		/// ## C++ default parameters
		/// * sf: 1.1
		/// * min_n: 3
		/// * min_sz: Size(30,30)
		/// * max_sz: Size()
		#[inline]
		pub fn new(cascade_model: &str, sf: f64, min_n: i32, min_sz: core::Size, max_sz: core::Size) -> Result<crate::face::CParams> {
			extern_container_arg!(mut cascade_model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_CParams_CParams_String_double_int_Size_Size(cascade_model.opencv_as_extern_mut(), sf, min_n, min_sz.opencv_as_extern(), max_sz.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::CParams::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * sf: 1.1
		/// * min_n: 3
		/// * min_sz: Size(30,30)
		/// * max_sz: Size()
		#[inline]
		pub fn new_def(cascade_model: &str) -> Result<crate::face::CParams> {
			extern_container_arg!(mut cascade_model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_CParams_CParams_String(cascade_model.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::CParams::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for CParams {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CParams")
				.field("cascade", &crate::face::CParamsTraitConst::cascade(self))
				.field("scale_factor", &crate::face::CParamsTraitConst::scale_factor(self))
				.field("min_neighbors", &crate::face::CParamsTraitConst::min_neighbors(self))
				.field("min_size", &crate::face::CParamsTraitConst::min_size(self))
				.field("max_size", &crate::face::CParamsTraitConst::max_size(self))
				.field("face_cascade", &crate::face::CParamsTraitConst::face_cascade(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::EigenFaceRecognizer]
	pub trait EigenFaceRecognizerTraitConst: crate::face::BasicFaceRecognizerTraitConst {
		fn as_raw_EigenFaceRecognizer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::EigenFaceRecognizer]
	pub trait EigenFaceRecognizerTrait: crate::face::BasicFaceRecognizerTrait + crate::face::EigenFaceRecognizerTraitConst {
		fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void;
	
	}
	
	pub struct EigenFaceRecognizer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { EigenFaceRecognizer }
	
	impl Drop for EigenFaceRecognizer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_EigenFaceRecognizer_delete(self.as_raw_mut_EigenFaceRecognizer()) };
		}
	}
	
	unsafe impl Send for EigenFaceRecognizer {}
	
	impl core::AlgorithmTraitConst for EigenFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for EigenFaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizerTraitConst for EigenFaceRecognizer {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::BasicFaceRecognizerTrait for EigenFaceRecognizer {
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for EigenFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for EigenFaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::EigenFaceRecognizerTraitConst for EigenFaceRecognizer {
		#[inline] fn as_raw_EigenFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::EigenFaceRecognizerTrait for EigenFaceRecognizer {
		#[inline] fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl EigenFaceRecognizer {
		/// ## Parameters
		/// * num_components: The number of components (read: Eigenfaces) kept for this Principal
		/// Component Analysis. As a hint: There's no rule how many components (read: Eigenfaces) should be
		/// kept for good reconstruction capabilities. It is based on your input data, so experiment with the
		/// number. Keeping 80 components should almost always be sufficient.
		/// * threshold: The threshold applied in the prediction.
		/// 
		/// ### Notes:
		/// 
		/// *   Training and prediction must be done on grayscale images, use cvtColor to convert between the
		///    color spaces.
		/// *   **THE EIGENFACES METHOD MAKES THE ASSUMPTION, THAT THE TRAINING AND TEST IMAGES ARE OF EQUAL
		///    SIZE.** (caps-lock, because I got so many mails asking for this). You have to make sure your
		///    input data has the correct shape, else a meaningful exception is thrown. Use resize to resize
		///    the images.
		/// *   This model does not support updating.
		/// 
		/// ### Model internal data:
		/// 
		/// *   num_components see EigenFaceRecognizer::create.
		/// *   threshold see EigenFaceRecognizer::create.
		/// *   eigenvalues The eigenvalues for this Principal Component Analysis (ordered descending).
		/// *   eigenvectors The eigenvectors for this Principal Component Analysis (ordered by their
		///    eigenvalue).
		/// *   mean The sample mean calculated from the training data.
		/// *   projections The projections of the training data.
		/// *   labels The threshold applied in the prediction. If the distance to the nearest neighbor is
		///    larger than the threshold, this method returns -1.
		/// 
		/// ## C++ default parameters
		/// * num_components: 0
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create(num_components: i32, threshold: f64) -> Result<core::Ptr<crate::face::EigenFaceRecognizer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_EigenFaceRecognizer_create_int_double(num_components, threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::EigenFaceRecognizer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Parameters
		/// * num_components: The number of components (read: Eigenfaces) kept for this Principal
		/// Component Analysis. As a hint: There's no rule how many components (read: Eigenfaces) should be
		/// kept for good reconstruction capabilities. It is based on your input data, so experiment with the
		/// number. Keeping 80 components should almost always be sufficient.
		/// * threshold: The threshold applied in the prediction.
		/// 
		/// ### Notes:
		/// 
		/// *   Training and prediction must be done on grayscale images, use cvtColor to convert between the
		///    color spaces.
		/// *   **THE EIGENFACES METHOD MAKES THE ASSUMPTION, THAT THE TRAINING AND TEST IMAGES ARE OF EQUAL
		///    SIZE.** (caps-lock, because I got so many mails asking for this). You have to make sure your
		///    input data has the correct shape, else a meaningful exception is thrown. Use resize to resize
		///    the images.
		/// *   This model does not support updating.
		/// 
		/// ### Model internal data:
		/// 
		/// *   num_components see EigenFaceRecognizer::create.
		/// *   threshold see EigenFaceRecognizer::create.
		/// *   eigenvalues The eigenvalues for this Principal Component Analysis (ordered descending).
		/// *   eigenvectors The eigenvectors for this Principal Component Analysis (ordered by their
		///    eigenvalue).
		/// *   mean The sample mean calculated from the training data.
		/// *   projections The projections of the training data.
		/// *   labels The threshold applied in the prediction. If the distance to the nearest neighbor is
		///    larger than the threshold, this method returns -1.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * num_components: 0
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::EigenFaceRecognizer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_EigenFaceRecognizer_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::EigenFaceRecognizer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { EigenFaceRecognizer, core::Algorithm, cv_face_EigenFaceRecognizer_to_Algorithm }
	
	boxed_cast_base! { EigenFaceRecognizer, crate::face::BasicFaceRecognizer, cv_face_EigenFaceRecognizer_to_BasicFaceRecognizer }
	
	boxed_cast_base! { EigenFaceRecognizer, crate::face::FaceRecognizer, cv_face_EigenFaceRecognizer_to_FaceRecognizer }
	
	impl std::fmt::Debug for EigenFaceRecognizer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("EigenFaceRecognizer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FaceRecognizer]
	pub trait FaceRecognizerTraitConst: core::AlgorithmTraitConst {
		fn as_raw_FaceRecognizer(&self) -> *const c_void;
	
		/// Predicts a label and associated confidence (e.g. distance) for a given input image.
		/// 
		/// ## Parameters
		/// * src: Sample image to get a prediction from.
		/// * label: The predicted label for the given image.
		/// * confidence: Associated confidence (e.g. distance) for the predicted label.
		/// 
		/// The suffix const means that prediction does not affect the internal model state, so the method can
		/// be safely called from within different threads.
		/// 
		/// The following example shows how to get a prediction from a trained model:
		/// 
		/// ```C++
		/// using namespace cv;
		/// // Do your initialization here (create the cv::FaceRecognizer model) ...
		/// // ...
		/// // Read in a sample image:
		/// Mat img = imread("person1/3.jpg", IMREAD_GRAYSCALE);
		/// // And get a prediction from the cv::FaceRecognizer:
		/// int predicted = model->predict(img);
		/// ```
		/// 
		/// 
		/// Or to get a prediction and the associated confidence (e.g. distance):
		/// 
		/// ```C++
		/// using namespace cv;
		/// // Do your initialization here (create the cv::FaceRecognizer model) ...
		/// // ...
		/// Mat img = imread("person1/3.jpg", IMREAD_GRAYSCALE);
		/// // Some variables for the predicted label and associated confidence (e.g. distance):
		/// int predicted_label = -1;
		/// double predicted_confidence = 0.0;
		/// // Get the prediction and associated confidence from the model
		/// model->predict(img, predicted_label, predicted_confidence);
		/// ```
		/// 
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn predict_label(&self, src: &impl core::ToInputArray) -> Result<i32> {
			input_array_arg!(src);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Predicts a label and associated confidence (e.g. distance) for a given input image.
		/// 
		/// ## Parameters
		/// * src: Sample image to get a prediction from.
		/// * label: The predicted label for the given image.
		/// * confidence: Associated confidence (e.g. distance) for the predicted label.
		/// 
		/// The suffix const means that prediction does not affect the internal model state, so the method can
		/// be safely called from within different threads.
		/// 
		/// The following example shows how to get a prediction from a trained model:
		/// 
		/// ```C++
		/// using namespace cv;
		/// // Do your initialization here (create the cv::FaceRecognizer model) ...
		/// // ...
		/// // Read in a sample image:
		/// Mat img = imread("person1/3.jpg", IMREAD_GRAYSCALE);
		/// // And get a prediction from the cv::FaceRecognizer:
		/// int predicted = model->predict(img);
		/// ```
		/// 
		/// 
		/// Or to get a prediction and the associated confidence (e.g. distance):
		/// 
		/// ```C++
		/// using namespace cv;
		/// // Do your initialization here (create the cv::FaceRecognizer model) ...
		/// // ...
		/// Mat img = imread("person1/3.jpg", IMREAD_GRAYSCALE);
		/// // Some variables for the predicted label and associated confidence (e.g. distance):
		/// int predicted_label = -1;
		/// double predicted_confidence = 0.0;
		/// // Get the prediction and associated confidence from the model
		/// model->predict(img, predicted_label, predicted_confidence);
		/// ```
		/// 
		#[inline]
		fn predict(&self, src: &impl core::ToInputArray, label: &mut i32, confidence: &mut f64) -> Result<()> {
			input_array_arg!(src);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR_intR_doubleR(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), label, confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// - if implemented - send all result of prediction to collector that can be used for somehow custom result handling
		/// ## Parameters
		/// * src: Sample image to get a prediction from.
		/// * collector: User-defined collector object that accepts all results
		/// 
		/// To implement this method u just have to do same internal cycle as in predict(InputArray src, CV_OUT int &label, CV_OUT double &confidence) but
		/// not try to get "best@ result, just resend it to caller side with given collector
		#[inline]
		fn predict_collect(&self, src: &impl core::ToInputArray, mut collector: core::Ptr<crate::face::PredictCollector>) -> Result<()> {
			input_array_arg!(src);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR_PtrLPredictCollectorG(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), collector.as_raw_mut_PtrOfPredictCollector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Saves a FaceRecognizer and its model state.
		/// 
		/// Saves this model to a given filename, either as XML or YAML.
		/// ## Parameters
		/// * filename: The filename to store this FaceRecognizer to (either XML/YAML).
		/// 
		/// Every FaceRecognizer overwrites FaceRecognizer::save(FileStorage& fs) to save the internal model
		/// state. FaceRecognizer::save(const String& filename) saves the state of a model to the given
		/// filename.
		/// 
		/// The suffix const means that prediction does not affect the internal model state, so the method can
		/// be safely called from within different threads.
		#[inline]
		fn write(&self, filename: &str) -> Result<()> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_write_const_const_StringR(self.as_raw_FaceRecognizer(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Saves a FaceRecognizer and its model state.
		/// 
		/// Saves this model to a given filename, either as XML or YAML.
		/// ## Parameters
		/// * filename: The filename to store this FaceRecognizer to (either XML/YAML).
		/// 
		/// Every FaceRecognizer overwrites FaceRecognizer::save(FileStorage& fs) to save the internal model
		/// state. FaceRecognizer::save(const String& filename) saves the state of a model to the given
		/// filename.
		/// 
		/// The suffix const means that prediction does not affect the internal model state, so the method can
		/// be safely called from within different threads.
		/// 
		/// ## Overloaded parameters
		/// 
		///    Saves this model to a given FileStorage.
		/// * fs: The FileStorage to store this FaceRecognizer to.
		#[inline]
		fn write_1(&self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_write_const_FileStorageR(self.as_raw_FaceRecognizer(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_empty_const(self.as_raw_FaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Gets string information by label.
		/// 
		/// If an unknown label id is provided or there is no label information associated with the specified
		/// label id the method returns an empty string.
		#[inline]
		fn get_label_info(&self, label: i32) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_getLabelInfo_const_int(self.as_raw_FaceRecognizer(), label, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Gets vector of labels by string.
		/// 
		/// The function searches for the labels containing the specified sub-string in the associated string
		/// info.
		#[inline]
		fn get_labels_by_string(&self, str: &str) -> Result<core::Vector<i32>> {
			extern_container_arg!(str);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_getLabelsByString_const_const_StringR(self.as_raw_FaceRecognizer(), str.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// threshold parameter accessor - required for default BestMinDist collector
		#[inline]
		fn get_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_getThreshold_const(self.as_raw_FaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::FaceRecognizer]
	pub trait FaceRecognizerTrait: core::AlgorithmTrait + crate::face::FaceRecognizerTraitConst {
		fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void;
	
		/// Trains a FaceRecognizer with given data and associated labels.
		/// 
		/// ## Parameters
		/// * src: The training images, that means the faces you want to learn. The data has to be
		/// given as a vector\<Mat\>.
		/// * labels: The labels corresponding to the images have to be given either as a vector\<int\>
		/// or a Mat of type CV_32SC1.
		/// 
		/// The following source code snippet shows you how to learn a Fisherfaces model on a given set of
		/// images. The images are read with imread and pushed into a std::vector\<Mat\>. The labels of each
		/// image are stored within a std::vector\<int\> (you could also use a Mat of type CV_32SC1). Think of
		/// the label as the subject (the person) this image belongs to, so same subjects (persons) should have
		/// the same label. For the available FaceRecognizer you don't have to pay any attention to the order of
		/// the labels, just make sure same persons have the same label:
		/// 
		/// ```C++
		/// // holds images and labels
		/// vector<Mat> images;
		/// vector<int> labels;
		/// // using Mat of type CV_32SC1
		/// // Mat labels(number_of_samples, 1, CV_32SC1);
		/// // images for first person
		/// images.push_back(imread("person0/0.jpg", IMREAD_GRAYSCALE)); labels.push_back(0);
		/// images.push_back(imread("person0/1.jpg", IMREAD_GRAYSCALE)); labels.push_back(0);
		/// images.push_back(imread("person0/2.jpg", IMREAD_GRAYSCALE)); labels.push_back(0);
		/// // images for second person
		/// images.push_back(imread("person1/0.jpg", IMREAD_GRAYSCALE)); labels.push_back(1);
		/// images.push_back(imread("person1/1.jpg", IMREAD_GRAYSCALE)); labels.push_back(1);
		/// images.push_back(imread("person1/2.jpg", IMREAD_GRAYSCALE)); labels.push_back(1);
		/// ```
		/// 
		/// 
		/// Now that you have read some images, we can create a new FaceRecognizer. In this example I'll create
		/// a Fisherfaces model and decide to keep all of the possible Fisherfaces:
		/// 
		/// ```C++
		/// // Create a new Fisherfaces model and retain all available Fisherfaces,
		/// // this is the most common usage of this specific FaceRecognizer:
		/// //
		/// Ptr<FaceRecognizer> model =  FisherFaceRecognizer::create();
		/// ```
		/// 
		/// 
		/// And finally train it on the given dataset (the face images and labels):
		/// 
		/// ```C++
		/// // This is the common interface to train all of the available cv::FaceRecognizer
		/// // implementations:
		/// //
		/// model->train(images, labels);
		/// ```
		/// 
		#[inline]
		fn train(&mut self, src: &impl core::ToInputArray, labels: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(src);
			input_array_arg!(labels);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_train_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Updates a FaceRecognizer with given data and associated labels.
		/// 
		/// ## Parameters
		/// * src: The training images, that means the faces you want to learn. The data has to be given
		/// as a vector\<Mat\>.
		/// * labels: The labels corresponding to the images have to be given either as a vector\<int\> or
		/// a Mat of type CV_32SC1.
		/// 
		/// This method updates a (probably trained) FaceRecognizer, but only if the algorithm supports it. The
		/// Local Binary Patterns Histograms (LBPH) recognizer (see createLBPHFaceRecognizer) can be updated.
		/// For the Eigenfaces and Fisherfaces method, this is algorithmically not possible and you have to
		/// re-estimate the model with FaceRecognizer::train. In any case, a call to train empties the existing
		/// model and learns a new model, while update does not delete any model data.
		/// 
		/// ```C++
		/// // Create a new LBPH model (it can be updated) and use the default parameters,
		/// // this is the most common usage of this specific FaceRecognizer:
		/// //
		/// Ptr<FaceRecognizer> model =  LBPHFaceRecognizer::create();
		/// // This is the common interface to train all of the available cv::FaceRecognizer
		/// // implementations:
		/// //
		/// model->train(images, labels);
		/// // Some containers to hold new image:
		/// vector<Mat> newImages;
		/// vector<int> newLabels;
		/// // You should add some images to the containers:
		/// //
		/// // ...
		/// //
		/// // Now updating the model is as easy as calling:
		/// model->update(newImages,newLabels);
		/// // This will preserve the old model data and extend the existing model
		/// // with the new features extracted from newImages!
		/// ```
		/// 
		/// 
		/// Calling update on an Eigenfaces model (see EigenFaceRecognizer::create), which doesn't support
		/// updating, will throw an error similar to:
		/// 
		/// ```C++
		/// OpenCV Error: The function/feature is not implemented (This FaceRecognizer (FaceRecognizer.Eigenfaces) does not support updating, you have to use FaceRecognizer::train to update it.) in update, file /home/philipp/git/opencv/modules/contrib/src/facerec.cpp, line 305
		/// terminate called after throwing an instance of "cv::Exception"
		/// ```
		/// 
		/// 
		/// 
		/// Note: The FaceRecognizer does not store your training images, because this would be very
		/// memory intense and it's not the responsibility of te FaceRecognizer to do so. The caller is
		/// responsible for maintaining the dataset, he want to work with.
		#[inline]
		fn update(&mut self, src: &impl core::ToInputArray, labels: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(src);
			input_array_arg!(labels);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_update_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Loads a FaceRecognizer and its model state.
		/// 
		/// Loads a persisted model and state from a given XML or YAML file . Every FaceRecognizer has to
		/// overwrite FaceRecognizer::load(FileStorage& fs) to enable loading the model state.
		/// FaceRecognizer::load(FileStorage& fs) in turn gets called by
		/// FaceRecognizer::load(const String& filename), to ease saving a model.
		#[inline]
		fn read(&mut self, filename: &str) -> Result<()> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_read_const_StringR(self.as_raw_mut_FaceRecognizer(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Loads a FaceRecognizer and its model state.
		/// 
		/// Loads a persisted model and state from a given XML or YAML file . Every FaceRecognizer has to
		/// overwrite FaceRecognizer::load(FileStorage& fs) to enable loading the model state.
		/// FaceRecognizer::load(FileStorage& fs) in turn gets called by
		/// FaceRecognizer::load(const String& filename), to ease saving a model.
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn read_1(&mut self, fn_: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_read_const_FileNodeR(self.as_raw_mut_FaceRecognizer(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Sets string info for the specified model's label.
		/// 
		/// The string info is replaced by the provided value if it was set before for the specified label.
		#[inline]
		fn set_label_info(&mut self, label: i32, str_info: &str) -> Result<()> {
			extern_container_arg!(str_info);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_setLabelInfo_int_const_StringR(self.as_raw_mut_FaceRecognizer(), label, str_info.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Sets threshold of model
		#[inline]
		fn set_threshold(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FaceRecognizer_setThreshold_double(self.as_raw_mut_FaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for all face recognition models
	/// 
	/// All face recognition models in OpenCV are derived from the abstract base class FaceRecognizer, which
	/// provides a unified access to all face recongition algorithms in OpenCV.
	/// 
	/// ### Description
	/// 
	/// I'll go a bit more into detail explaining FaceRecognizer, because it doesn't look like a powerful
	/// interface at first sight. But: Every FaceRecognizer is an Algorithm, so you can easily get/set all
	/// model internals (if allowed by the implementation). Algorithm is a relatively new OpenCV concept,
	/// which is available since the 2.4 release. I suggest you take a look at its description.
	/// 
	/// Algorithm provides the following features for all derived classes:
	/// 
	/// *   So called "virtual constructor". That is, each Algorithm derivative is registered at program
	///    start and you can get the list of registered algorithms and create instance of a particular
	///    algorithm by its name (see Algorithm::create). If you plan to add your own algorithms, it is
	///    good practice to add a unique prefix to your algorithms to distinguish them from other
	///    algorithms.
	/// *   Setting/Retrieving algorithm parameters by name. If you used video capturing functionality from
	///    OpenCV highgui module, you are probably familar with cv::cvSetCaptureProperty,
	/// ocvcvGetCaptureProperty, VideoCapture::set and VideoCapture::get. Algorithm provides similar
	///    method where instead of integer id's you specify the parameter names as text Strings. See
	///    Algorithm::set and Algorithm::get for details.
	/// *   Reading and writing parameters from/to XML or YAML files. Every Algorithm derivative can store
	///    all its parameters and then read them back. There is no need to re-implement it each time.
	/// 
	/// Moreover every FaceRecognizer supports the:
	/// 
	/// *   **Training** of a FaceRecognizer with FaceRecognizer::train on a given set of images (your face
	///    database!).
	/// *   **Prediction** of a given sample image, that means a face. The image is given as a Mat.
	/// *   **Loading/Saving** the model state from/to a given XML or YAML.
	/// *   **Setting/Getting labels info**, that is stored as a string. String labels info is useful for
	///    keeping names of the recognized people.
	/// 
	/// 
	/// Note: When using the FaceRecognizer interface in combination with Python, please stick to Python 2.
	/// Some underlying scripts like create_csv will not work in other versions, like Python 3. Setting the
	/// Thresholds +++++++++++++++++++++++
	/// 
	/// Sometimes you run into the situation, when you want to apply a threshold on the prediction. A common
	/// scenario in face recognition is to tell, whether a face belongs to the training dataset or if it is
	/// unknown. You might wonder, why there's no public API in FaceRecognizer to set the threshold for the
	/// prediction, but rest assured: It's supported. It just means there's no generic way in an abstract
	/// class to provide an interface for setting/getting the thresholds of *every possible* FaceRecognizer
	/// algorithm. The appropriate place to set the thresholds is in the constructor of the specific
	/// FaceRecognizer and since every FaceRecognizer is a Algorithm (see above), you can get/set the
	/// thresholds at runtime!
	/// 
	/// Here is an example of setting a threshold for the Eigenfaces method, when creating the model:
	/// 
	/// ```C++
	/// // Let's say we want to keep 10 Eigenfaces and have a threshold value of 10.0
	/// int num_components = 10;
	/// double threshold = 10.0;
	/// // Then if you want to have a cv::FaceRecognizer with a confidence threshold,
	/// // create the concrete implementation with the appropriate parameters:
	/// Ptr<FaceRecognizer> model = EigenFaceRecognizer::create(num_components, threshold);
	/// ```
	/// 
	/// 
	/// Sometimes it's impossible to train the model, just to experiment with threshold values. Thanks to
	/// Algorithm it's possible to set internal model thresholds during runtime. Let's see how we would
	/// set/get the prediction for the Eigenface model, we've created above:
	/// 
	/// ```C++
	/// // The following line reads the threshold from the Eigenfaces model:
	/// double current_threshold = model->getDouble("threshold");
	/// // And this line sets the threshold to 0.0:
	/// model->set("threshold", 0.0);
	/// ```
	/// 
	/// 
	/// If you've set the threshold to 0.0 as we did above, then:
	/// 
	/// ```C++
	/// //
	/// Mat img = imread("person1/3.jpg", IMREAD_GRAYSCALE);
	/// // Get a prediction from the model. Note: We've set a threshold of 0.0 above,
	/// // since the distance is almost always larger than 0.0, you'll get -1 as
	/// // label, which indicates, this face is unknown
	/// int predicted_label = model->predict(img);
	/// // ...
	/// ```
	/// 
	/// 
	/// is going to yield -1 as predicted label, which states this face is unknown.
	/// 
	/// ### Getting the name of a FaceRecognizer
	/// 
	/// Since every FaceRecognizer is a Algorithm, you can use Algorithm::name to get the name of a
	/// FaceRecognizer:
	/// 
	/// ```C++
	/// // Create a FaceRecognizer:
	/// Ptr<FaceRecognizer> model = EigenFaceRecognizer::create();
	/// // And here's how to get its name:
	/// String name = model->name();
	/// ```
	/// 
	pub struct FaceRecognizer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FaceRecognizer }
	
	impl Drop for FaceRecognizer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FaceRecognizer_delete(self.as_raw_mut_FaceRecognizer()) };
		}
	}
	
	unsafe impl Send for FaceRecognizer {}
	
	impl core::AlgorithmTraitConst for FaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for FaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for FaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FaceRecognizer {
	}
	
	boxed_cast_descendant! { FaceRecognizer, crate::face::BasicFaceRecognizer, cv_face_FaceRecognizer_to_BasicFaceRecognizer }
	
	boxed_cast_descendant! { FaceRecognizer, crate::face::EigenFaceRecognizer, cv_face_FaceRecognizer_to_EigenFaceRecognizer }
	
	boxed_cast_descendant! { FaceRecognizer, crate::face::FisherFaceRecognizer, cv_face_FaceRecognizer_to_FisherFaceRecognizer }
	
	boxed_cast_descendant! { FaceRecognizer, crate::face::LBPHFaceRecognizer, cv_face_FaceRecognizer_to_LBPHFaceRecognizer }
	
	boxed_cast_base! { FaceRecognizer, core::Algorithm, cv_face_FaceRecognizer_to_Algorithm }
	
	impl std::fmt::Debug for FaceRecognizer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FaceRecognizer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::Facemark]
	pub trait FacemarkTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Facemark(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::Facemark]
	pub trait FacemarkTrait: core::AlgorithmTrait + crate::face::FacemarkTraitConst {
		fn as_raw_mut_Facemark(&mut self) -> *mut c_void;
	
		/// A function to load the trained model before the fitting process.
		/// ## Parameters
		/// * model: A string represent the filename of a trained model.
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// facemark->loadModel("../data/lbf.model");
		/// ```
		/// 
		#[inline]
		fn load_model(&mut self, model: &str) -> Result<()> {
			extern_container_arg!(mut model);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_Facemark_loadModel_String(self.as_raw_mut_Facemark(), model.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detect facial landmarks from an image.
		/// ## Parameters
		/// * image: Input image.
		/// * faces: Output of the function which represent region of interest of the detected faces.
		/// Each face is stored in cv::Rect container.
		/// * landmarks: The detected landmark points for each faces.
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// Mat image = imread("image.jpg");
		/// std::vector<Rect> faces;
		/// std::vector<std::vector<Point2f> > landmarks;
		/// facemark->fit(image, faces, landmarks);
		/// ```
		/// 
		#[inline]
		fn fit(&mut self, image: &impl core::ToInputArray, faces: &impl core::ToInputArray, landmarks: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(image);
			input_array_arg!(faces);
			output_array_arg!(landmarks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_Facemark_fit_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Facemark(), image.as_raw__InputArray(), faces.as_raw__InputArray(), landmarks.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for all facemark models
	/// 
	/// To utilize this API in your program, please take a look at the [tutorial_table_of_content_facemark]
	/// ### Description
	/// 
	/// Facemark is a base class which provides universal access to any specific facemark algorithm.
	/// Therefore, the users should declare a desired algorithm before they can use it in their application.
	/// 
	/// Here is an example on how to declare a facemark algorithm:
	/// ```C++
	/// // Using Facemark in your code:
	/// Ptr<Facemark> facemark = createFacemarkLBF();
	/// ```
	/// 
	/// 
	/// The typical pipeline for facemark detection is as follows:
	/// - Load the trained model using Facemark::loadModel.
	/// - Perform the fitting on an image via Facemark::fit.
	pub struct Facemark {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Facemark }
	
	impl Drop for Facemark {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_Facemark_delete(self.as_raw_mut_Facemark()) };
		}
	}
	
	unsafe impl Send for Facemark {}
	
	impl core::AlgorithmTraitConst for Facemark {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Facemark {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for Facemark {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrait for Facemark {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Facemark {
	}
	
	boxed_cast_descendant! { Facemark, crate::face::FacemarkAAM, cv_face_Facemark_to_FacemarkAAM }
	
	boxed_cast_descendant! { Facemark, crate::face::FacemarkKazemi, cv_face_Facemark_to_FacemarkKazemi }
	
	boxed_cast_descendant! { Facemark, crate::face::FacemarkLBF, cv_face_Facemark_to_FacemarkLBF }
	
	boxed_cast_descendant! { Facemark, crate::face::FacemarkTrain, cv_face_Facemark_to_FacemarkTrain }
	
	boxed_cast_base! { Facemark, core::Algorithm, cv_face_Facemark_to_Algorithm }
	
	impl std::fmt::Debug for Facemark {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Facemark")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkAAM]
	pub trait FacemarkAAMTraitConst: crate::face::FacemarkTrainTraitConst {
		fn as_raw_FacemarkAAM(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::FacemarkAAM]
	pub trait FacemarkAAMTrait: crate::face::FacemarkAAMTraitConst + crate::face::FacemarkTrainTrait {
		fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void;
	
		/// overload with additional Config structures
		#[inline]
		fn fit_config(&mut self, image: &impl core::ToInputArray, roi: &impl core::ToInputArray, _landmarks: &mut impl core::ToOutputArray, runtime_params: &core::Vector<crate::face::FacemarkAAM_Config>) -> Result<bool> {
			input_array_arg!(image);
			input_array_arg!(roi);
			output_array_arg!(_landmarks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_fitConfig_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_vectorLConfigGR(self.as_raw_mut_FacemarkAAM(), image.as_raw__InputArray(), roi.as_raw__InputArray(), _landmarks.as_raw__OutputArray(), runtime_params.as_raw_VectorOfFacemarkAAM_Config(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct FacemarkAAM {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkAAM }
	
	impl Drop for FacemarkAAM {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkAAM_delete(self.as_raw_mut_FacemarkAAM()) };
		}
	}
	
	unsafe impl Send for FacemarkAAM {}
	
	impl core::AlgorithmTraitConst for FacemarkAAM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FacemarkAAM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for FacemarkAAM {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrait for FacemarkAAM {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrainTraitConst for FacemarkAAM {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrainTrait for FacemarkAAM {
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkAAMTraitConst for FacemarkAAM {
		#[inline] fn as_raw_FacemarkAAM(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkAAMTrait for FacemarkAAM {
		#[inline] fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkAAM {
		/// initializer
		/// 
		/// ## C++ default parameters
		/// * parameters: FacemarkAAM::Params()
		#[inline]
		pub fn create(parameters: &crate::face::FacemarkAAM_Params) -> Result<core::Ptr<crate::face::FacemarkAAM>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_create_const_ParamsR(parameters.as_raw_FacemarkAAM_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FacemarkAAM>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// initializer
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * parameters: FacemarkAAM::Params()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::FacemarkAAM>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FacemarkAAM>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { FacemarkAAM, core::Algorithm, cv_face_FacemarkAAM_to_Algorithm }
	
	boxed_cast_base! { FacemarkAAM, crate::face::Facemark, cv_face_FacemarkAAM_to_Facemark }
	
	boxed_cast_base! { FacemarkAAM, crate::face::FacemarkTrain, cv_face_FacemarkAAM_to_FacemarkTrain }
	
	impl std::fmt::Debug for FacemarkAAM {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkAAM")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkAAM_Config]
	pub trait FacemarkAAM_ConfigTraitConst {
		fn as_raw_FacemarkAAM_Config(&self) -> *const c_void;
	
		#[inline]
		fn r(&self) -> core::Mat {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Config_propR_const(self.as_raw_FacemarkAAM_Config()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn t(&self) -> core::Point2f {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_Config_propT_const(self.as_raw_FacemarkAAM_Config(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn scale(&self) -> f32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Config_propScale_const(self.as_raw_FacemarkAAM_Config()) };
			ret
		}
		
		#[inline]
		fn model_scale_idx(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Config_propModel_scale_idx_const(self.as_raw_FacemarkAAM_Config()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::face::FacemarkAAM_Config]
	pub trait FacemarkAAM_ConfigTrait: crate::face::FacemarkAAM_ConfigTraitConst {
		fn as_raw_mut_FacemarkAAM_Config(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_r(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Config_propR_Mat(self.as_raw_mut_FacemarkAAM_Config(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_t(&mut self, val: core::Point2f) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Config_propT_Point2f(self.as_raw_mut_FacemarkAAM_Config(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_scale(&mut self, val: f32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Config_propScale_float(self.as_raw_mut_FacemarkAAM_Config(), val) };
			ret
		}
		
		#[inline]
		fn set_model_scale_idx(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Config_propModel_scale_idx_int(self.as_raw_mut_FacemarkAAM_Config(), val) };
			ret
		}
		
	}
	
	/// \brief Optional parameter for fitting process.
	pub struct FacemarkAAM_Config {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkAAM_Config }
	
	impl Drop for FacemarkAAM_Config {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkAAM_Config_delete(self.as_raw_mut_FacemarkAAM_Config()) };
		}
	}
	
	unsafe impl Send for FacemarkAAM_Config {}
	
	impl crate::face::FacemarkAAM_ConfigTraitConst for FacemarkAAM_Config {
		#[inline] fn as_raw_FacemarkAAM_Config(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkAAM_ConfigTrait for FacemarkAAM_Config {
		#[inline] fn as_raw_mut_FacemarkAAM_Config(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkAAM_Config {
		/// ## C++ default parameters
		/// * rot: Mat::eye(2,2,CV_32F)
		/// * trans: Point2f(0.0f,0.0f)
		/// * scaling: 1.0f
		/// * scale_id: 0
		#[inline]
		pub fn new(mut rot: core::Mat, trans: core::Point2f, scaling: f32, scale_id: i32) -> Result<crate::face::FacemarkAAM_Config> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(rot.as_raw_mut_Mat(), trans.opencv_as_extern(), scaling, scale_id, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::FacemarkAAM_Config::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * rot: Mat::eye(2,2,CV_32F)
		/// * trans: Point2f(0.0f,0.0f)
		/// * scaling: 1.0f
		/// * scale_id: 0
		#[inline]
		pub fn new_def() -> Result<crate::face::FacemarkAAM_Config> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_Config_Config(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::FacemarkAAM_Config::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for FacemarkAAM_Config {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkAAM_Config")
				.field("r", &crate::face::FacemarkAAM_ConfigTraitConst::r(self))
				.field("t", &crate::face::FacemarkAAM_ConfigTraitConst::t(self))
				.field("scale", &crate::face::FacemarkAAM_ConfigTraitConst::scale(self))
				.field("model_scale_idx", &crate::face::FacemarkAAM_ConfigTraitConst::model_scale_idx(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkAAM_Data]
	pub trait FacemarkAAM_DataTraitConst {
		fn as_raw_FacemarkAAM_Data(&self) -> *const c_void;
	
		#[inline]
		fn s0(&self) -> core::Vector<core::Point2f> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Data_propS0_const(self.as_raw_FacemarkAAM_Data()) };
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::face::FacemarkAAM_Data]
	pub trait FacemarkAAM_DataTrait: crate::face::FacemarkAAM_DataTraitConst {
		fn as_raw_mut_FacemarkAAM_Data(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_s0(&mut self, mut val: core::Vector<core::Point2f>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Data_propS0_vectorLPoint2fG(self.as_raw_mut_FacemarkAAM_Data(), val.as_raw_mut_VectorOfPoint2f()) };
			ret
		}
		
	}
	
	/// \brief Data container for the facemark::getData function
	pub struct FacemarkAAM_Data {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkAAM_Data }
	
	impl Drop for FacemarkAAM_Data {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkAAM_Data_delete(self.as_raw_mut_FacemarkAAM_Data()) };
		}
	}
	
	unsafe impl Send for FacemarkAAM_Data {}
	
	impl crate::face::FacemarkAAM_DataTraitConst for FacemarkAAM_Data {
		#[inline] fn as_raw_FacemarkAAM_Data(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkAAM_DataTrait for FacemarkAAM_Data {
		#[inline] fn as_raw_mut_FacemarkAAM_Data(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkAAM_Data {
	}
	
	impl std::fmt::Debug for FacemarkAAM_Data {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkAAM_Data")
				.field("s0", &crate::face::FacemarkAAM_DataTraitConst::s0(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkAAM_Model]
	pub trait FacemarkAAM_ModelTraitConst {
		fn as_raw_FacemarkAAM_Model(&self) -> *const c_void;
	
		#[inline]
		fn scales(&self) -> core::Vector<f32> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propScales_const(self.as_raw_FacemarkAAM_Model()) };
			let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn triangles(&self) -> core::Vector<core::Vec3i> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propTriangles_const(self.as_raw_FacemarkAAM_Model()) };
			let ret = unsafe { core::Vector::<core::Vec3i>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn textures(&self) -> core::Vector<crate::face::FacemarkAAM_Model_Texture> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propTextures_const(self.as_raw_FacemarkAAM_Model()) };
			let ret = unsafe { core::Vector::<crate::face::FacemarkAAM_Model_Texture>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn s0(&self) -> core::Vector<core::Point2f> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propS0_const(self.as_raw_FacemarkAAM_Model()) };
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn s(&self) -> core::Mat {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propS_const(self.as_raw_FacemarkAAM_Model()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn q(&self) -> core::Mat {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propQ_const(self.as_raw_FacemarkAAM_Model()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::face::FacemarkAAM_Model]
	pub trait FacemarkAAM_ModelTrait: crate::face::FacemarkAAM_ModelTraitConst {
		fn as_raw_mut_FacemarkAAM_Model(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_scales(&mut self, mut val: core::Vector<f32>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propScales_vectorLfloatG(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOff32()) };
			ret
		}
		
		#[inline]
		fn set_triangles(&mut self, mut val: core::Vector<core::Vec3i>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propTriangles_vectorLVec3iG(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfVec3i()) };
			ret
		}
		
		#[inline]
		fn set_textures(&mut self, mut val: core::Vector<crate::face::FacemarkAAM_Model_Texture>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propTextures_vectorLTextureG(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfFacemarkAAM_Model_Texture()) };
			ret
		}
		
		#[inline]
		fn set_s0(&mut self, mut val: core::Vector<core::Point2f>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propS0_vectorLPoint2fG(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfPoint2f()) };
			ret
		}
		
		#[inline]
		fn set_s(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propS_Mat(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_q(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_propQ_Mat(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_Mat()) };
			ret
		}
		
	}
	
	/// \brief The model of AAM Algorithm
	pub struct FacemarkAAM_Model {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkAAM_Model }
	
	impl Drop for FacemarkAAM_Model {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkAAM_Model_delete(self.as_raw_mut_FacemarkAAM_Model()) };
		}
	}
	
	unsafe impl Send for FacemarkAAM_Model {}
	
	impl crate::face::FacemarkAAM_ModelTraitConst for FacemarkAAM_Model {
		#[inline] fn as_raw_FacemarkAAM_Model(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkAAM_ModelTrait for FacemarkAAM_Model {
		#[inline] fn as_raw_mut_FacemarkAAM_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkAAM_Model {
	}
	
	impl std::fmt::Debug for FacemarkAAM_Model {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkAAM_Model")
				.field("scales", &crate::face::FacemarkAAM_ModelTraitConst::scales(self))
				.field("triangles", &crate::face::FacemarkAAM_ModelTraitConst::triangles(self))
				.field("textures", &crate::face::FacemarkAAM_ModelTraitConst::textures(self))
				.field("s0", &crate::face::FacemarkAAM_ModelTraitConst::s0(self))
				.field("s", &crate::face::FacemarkAAM_ModelTraitConst::s(self))
				.field("q", &crate::face::FacemarkAAM_ModelTraitConst::q(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkAAM_Model_Texture]
	pub trait FacemarkAAM_Model_TextureTraitConst {
		fn as_raw_FacemarkAAM_Model_Texture(&self) -> *const c_void;
	
		/// unused delete
		#[inline]
		fn max_m(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propMax_m_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			ret
		}
		
		#[inline]
		fn resolution(&self) -> core::Rect {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propResolution_const(self.as_raw_FacemarkAAM_Model_Texture(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn a(&self) -> core::Mat {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propA_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn a0(&self) -> core::Mat {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propA0_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn aa(&self) -> core::Mat {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propAA_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn aa0(&self) -> core::Mat {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propAA0_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn texture_idx(&self) -> core::Vector<core::Vector<core::Point>> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propTextureIdx_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Vector::<core::Vector<core::Point>>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn base_shape(&self) -> core::Vector<core::Point2f> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propBase_shape_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn ind1(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propInd1_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn ind2(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propInd2_const(self.as_raw_FacemarkAAM_Model_Texture()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::face::FacemarkAAM_Model_Texture]
	pub trait FacemarkAAM_Model_TextureTrait: crate::face::FacemarkAAM_Model_TextureTraitConst {
		fn as_raw_mut_FacemarkAAM_Model_Texture(&mut self) -> *mut c_void;
	
		/// unused delete
		#[inline]
		fn set_max_m(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propMax_m_int(self.as_raw_mut_FacemarkAAM_Model_Texture(), val) };
			ret
		}
		
		#[inline]
		fn set_resolution(&mut self, val: core::Rect) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propResolution_Rect(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_a(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propA_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_a0(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propA0_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_aa(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propAA_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_aa0(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propAA0_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) };
			ret
		}
		
		#[inline]
		fn set_texture_idx(&mut self, mut val: core::Vector<core::Vector<core::Point>>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propTextureIdx_vectorLvectorLPointGG(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfVectorOfPoint()) };
			ret
		}
		
		#[inline]
		fn set_base_shape(&mut self, mut val: core::Vector<core::Point2f>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propBase_shape_vectorLPoint2fG(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfPoint2f()) };
			ret
		}
		
		#[inline]
		fn set_ind1(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propInd1_vectorLintG(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		#[inline]
		fn set_ind2(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Model_Texture_propInd2_vectorLintG(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
	}
	
	pub struct FacemarkAAM_Model_Texture {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkAAM_Model_Texture }
	
	impl Drop for FacemarkAAM_Model_Texture {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkAAM_Model_Texture_delete(self.as_raw_mut_FacemarkAAM_Model_Texture()) };
		}
	}
	
	unsafe impl Send for FacemarkAAM_Model_Texture {}
	
	impl crate::face::FacemarkAAM_Model_TextureTraitConst for FacemarkAAM_Model_Texture {
		#[inline] fn as_raw_FacemarkAAM_Model_Texture(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkAAM_Model_TextureTrait for FacemarkAAM_Model_Texture {
		#[inline] fn as_raw_mut_FacemarkAAM_Model_Texture(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkAAM_Model_Texture {
	}
	
	impl std::fmt::Debug for FacemarkAAM_Model_Texture {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkAAM_Model_Texture")
				.field("max_m", &crate::face::FacemarkAAM_Model_TextureTraitConst::max_m(self))
				.field("resolution", &crate::face::FacemarkAAM_Model_TextureTraitConst::resolution(self))
				.field("a", &crate::face::FacemarkAAM_Model_TextureTraitConst::a(self))
				.field("a0", &crate::face::FacemarkAAM_Model_TextureTraitConst::a0(self))
				.field("aa", &crate::face::FacemarkAAM_Model_TextureTraitConst::aa(self))
				.field("aa0", &crate::face::FacemarkAAM_Model_TextureTraitConst::aa0(self))
				.field("texture_idx", &crate::face::FacemarkAAM_Model_TextureTraitConst::texture_idx(self))
				.field("base_shape", &crate::face::FacemarkAAM_Model_TextureTraitConst::base_shape(self))
				.field("ind1", &crate::face::FacemarkAAM_Model_TextureTraitConst::ind1(self))
				.field("ind2", &crate::face::FacemarkAAM_Model_TextureTraitConst::ind2(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkAAM_Params]
	pub trait FacemarkAAM_ParamsTraitConst {
		fn as_raw_FacemarkAAM_Params(&self) -> *const c_void;
	
		#[inline]
		fn model_filename(&self) -> String {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propModel_filename_const(self.as_raw_FacemarkAAM_Params()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn m(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propM_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn n(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propN_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn n_iter(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propN_iter_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn verbose(&self) -> bool {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propVerbose_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn save_model(&self) -> bool {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propSave_model_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn max_m(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propMax_m_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn max_n(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propMax_n_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn texture_max_m(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propTexture_max_m_const(self.as_raw_FacemarkAAM_Params()) };
			ret
		}
		
		#[inline]
		fn scales(&self) -> core::Vector<f32> {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propScales_const(self.as_raw_FacemarkAAM_Params()) };
			let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
			ret
		}
		
		/// \brief Read parameters from file, currently unused
		#[inline]
		fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_Params_write_const_FileStorageR(self.as_raw_FacemarkAAM_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::FacemarkAAM_Params]
	pub trait FacemarkAAM_ParamsTrait: crate::face::FacemarkAAM_ParamsTraitConst {
		fn as_raw_mut_FacemarkAAM_Params(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_model_filename(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propModel_filename_string(self.as_raw_mut_FacemarkAAM_Params(), val.opencv_as_extern_mut()) };
			ret
		}
		
		#[inline]
		fn set_m(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propM_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_n(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propN_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_n_iter(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propN_iter_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_verbose(&mut self, val: bool) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propVerbose_bool(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_save_model(&mut self, val: bool) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propSave_model_bool(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_max_m(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propMax_m_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_max_n(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propMax_n_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_texture_max_m(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propTexture_max_m_int(self.as_raw_mut_FacemarkAAM_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_scales(&mut self, mut val: core::Vector<f32>) {
			let ret = unsafe { sys::cv_face_FacemarkAAM_Params_propScales_vectorLfloatG(self.as_raw_mut_FacemarkAAM_Params(), val.as_raw_mut_VectorOff32()) };
			ret
		}
		
		/// \brief Read parameters from file, currently unused
		#[inline]
		fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_Params_read_const_FileNodeR(self.as_raw_mut_FacemarkAAM_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct FacemarkAAM_Params {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkAAM_Params }
	
	impl Drop for FacemarkAAM_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkAAM_Params_delete(self.as_raw_mut_FacemarkAAM_Params()) };
		}
	}
	
	unsafe impl Send for FacemarkAAM_Params {}
	
	impl crate::face::FacemarkAAM_ParamsTraitConst for FacemarkAAM_Params {
		#[inline] fn as_raw_FacemarkAAM_Params(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkAAM_ParamsTrait for FacemarkAAM_Params {
		#[inline] fn as_raw_mut_FacemarkAAM_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkAAM_Params {
		/// \brief Constructor
		#[inline]
		pub fn default() -> Result<crate::face::FacemarkAAM_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkAAM_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::FacemarkAAM_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for FacemarkAAM_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkAAM_Params")
				.field("model_filename", &crate::face::FacemarkAAM_ParamsTraitConst::model_filename(self))
				.field("m", &crate::face::FacemarkAAM_ParamsTraitConst::m(self))
				.field("n", &crate::face::FacemarkAAM_ParamsTraitConst::n(self))
				.field("n_iter", &crate::face::FacemarkAAM_ParamsTraitConst::n_iter(self))
				.field("verbose", &crate::face::FacemarkAAM_ParamsTraitConst::verbose(self))
				.field("save_model", &crate::face::FacemarkAAM_ParamsTraitConst::save_model(self))
				.field("max_m", &crate::face::FacemarkAAM_ParamsTraitConst::max_m(self))
				.field("max_n", &crate::face::FacemarkAAM_ParamsTraitConst::max_n(self))
				.field("texture_max_m", &crate::face::FacemarkAAM_ParamsTraitConst::texture_max_m(self))
				.field("scales", &crate::face::FacemarkAAM_ParamsTraitConst::scales(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkKazemi]
	pub trait FacemarkKazemiTraitConst: crate::face::FacemarkTraitConst {
		fn as_raw_FacemarkKazemi(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::FacemarkKazemi]
	pub trait FacemarkKazemiTrait: crate::face::FacemarkKazemiTraitConst + crate::face::FacemarkTrait {
		fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void;
	
		/// This function is used to train the model using gradient boosting to get a cascade of regressors
		/// which can then be used to predict shape.
		/// ## Parameters
		/// * images: A vector of type cv::Mat which stores the images which are used in training samples.
		/// * landmarks: A vector of vectors of type cv::Point2f which stores the landmarks detected in a particular image.
		/// * scale: A size of type cv::Size to which all images and landmarks have to be scaled to.
		/// * configfile: A variable of type std::string which stores the name of the file storing parameters for training the model.
		/// * modelFilename: A variable of type std::string which stores the name of the trained model file that has to be saved.
		/// ## Returns
		/// A boolean value. The function returns true if the model is trained properly or false if it is not trained.
		/// 
		/// ## C++ default parameters
		/// * model_filename: "face_landmarks.dat"
		#[inline]
		fn training(&mut self, images: &mut core::Vector<core::Mat>, landmarks: &mut core::Vector<core::Vector<core::Point2f>>, configfile: &str, scale: core::Size, model_filename: &str) -> Result<bool> {
			extern_container_arg!(mut configfile);
			extern_container_arg!(mut model_filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkKazemi_training_vectorLMatGR_vectorLvectorLPoint2fGGR_string_Size_string(self.as_raw_mut_FacemarkKazemi(), images.as_raw_mut_VectorOfMat(), landmarks.as_raw_mut_VectorOfVectorOfPoint2f(), configfile.opencv_as_extern_mut(), scale.opencv_as_extern(), model_filename.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// This function is used to train the model using gradient boosting to get a cascade of regressors
		/// which can then be used to predict shape.
		/// ## Parameters
		/// * images: A vector of type cv::Mat which stores the images which are used in training samples.
		/// * landmarks: A vector of vectors of type cv::Point2f which stores the landmarks detected in a particular image.
		/// * scale: A size of type cv::Size to which all images and landmarks have to be scaled to.
		/// * configfile: A variable of type std::string which stores the name of the file storing parameters for training the model.
		/// * modelFilename: A variable of type std::string which stores the name of the trained model file that has to be saved.
		/// ## Returns
		/// A boolean value. The function returns true if the model is trained properly or false if it is not trained.
		/// 
		/// ## Note
		/// This alternative version of [training] function uses the following default values for its arguments:
		/// * model_filename: "face_landmarks.dat"
		#[inline]
		fn training_def(&mut self, images: &mut core::Vector<core::Mat>, landmarks: &mut core::Vector<core::Vector<core::Point2f>>, configfile: &str, scale: core::Size) -> Result<bool> {
			extern_container_arg!(mut configfile);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkKazemi_training_vectorLMatGR_vectorLvectorLPoint2fGGR_string_Size(self.as_raw_mut_FacemarkKazemi(), images.as_raw_mut_VectorOfMat(), landmarks.as_raw_mut_VectorOfVectorOfPoint2f(), configfile.opencv_as_extern_mut(), scale.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set the custom face detector
		#[inline]
		fn set_face_detector(&mut self, f: Option<Box<dyn FnMut(*const c_void, *const c_void) -> bool + Send + Sync + 'static>>) -> Result<bool> {
			callback_arg!(f_trampoline(unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *mut c_void) -> bool => unnamed_2 in callbacks => f(unnamed: *const c_void, unnamed_1: *const c_void) -> bool);
			userdata_arg!(user_data in callbacks => f);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayR__const_cv__OutputArrayR__voidX__voidX(self.as_raw_mut_FacemarkKazemi(), f_trampoline, user_data, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// get faces using the custom detector
		#[inline]
		fn get_faces(&mut self, image: &impl core::ToInputArray, faces: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(image);
			output_array_arg!(faces);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkKazemi_getFaces_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FacemarkKazemi(), image.as_raw__InputArray(), faces.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct FacemarkKazemi {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkKazemi }
	
	impl Drop for FacemarkKazemi {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkKazemi_delete(self.as_raw_mut_FacemarkKazemi()) };
		}
	}
	
	unsafe impl Send for FacemarkKazemi {}
	
	impl core::AlgorithmTraitConst for FacemarkKazemi {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FacemarkKazemi {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for FacemarkKazemi {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrait for FacemarkKazemi {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkKazemiTraitConst for FacemarkKazemi {
		#[inline] fn as_raw_FacemarkKazemi(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkKazemiTrait for FacemarkKazemi {
		#[inline] fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkKazemi {
		/// ## C++ default parameters
		/// * parameters: FacemarkKazemi::Params()
		#[inline]
		pub fn create(parameters: &crate::face::FacemarkKazemi_Params) -> Result<core::Ptr<crate::face::FacemarkKazemi>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkKazemi_create_const_ParamsR(parameters.as_raw_FacemarkKazemi_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FacemarkKazemi>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * parameters: FacemarkKazemi::Params()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::FacemarkKazemi>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkKazemi_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FacemarkKazemi>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { FacemarkKazemi, core::Algorithm, cv_face_FacemarkKazemi_to_Algorithm }
	
	boxed_cast_base! { FacemarkKazemi, crate::face::Facemark, cv_face_FacemarkKazemi_to_Facemark }
	
	impl std::fmt::Debug for FacemarkKazemi {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkKazemi")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkKazemi_Params]
	pub trait FacemarkKazemi_ParamsTraitConst {
		fn as_raw_FacemarkKazemi_Params(&self) -> *const c_void;
	
		/// cascade_depth This stores the deapth of cascade used for training.
		#[inline]
		fn cascade_depth(&self) -> u32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propCascade_depth_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// tree_depth This stores the max height of the regression tree built.
		#[inline]
		fn tree_depth(&self) -> u32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propTree_depth_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// num_trees_per_cascade_level This stores number of trees fit per cascade level.
		#[inline]
		fn num_trees_per_cascade_level(&self) -> u32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propNum_trees_per_cascade_level_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// learning_rate stores the learning rate in gradient boosting, also referred as shrinkage.
		#[inline]
		fn learning_rate(&self) -> f32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propLearning_rate_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// oversampling_amount stores number of initialisations used to create training samples.
		#[inline]
		fn oversampling_amount(&self) -> u32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propOversampling_amount_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// num_test_coordinates stores number of test coordinates.
		#[inline]
		fn num_test_coordinates(&self) -> u32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propNum_test_coordinates_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// lambda stores a value to calculate probability of closeness of two coordinates.
		#[inline]
		fn lambda(&self) -> f32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propLambda_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// num_test_splits stores number of random test splits generated.
		#[inline]
		fn num_test_splits(&self) -> u32 {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propNum_test_splits_const(self.as_raw_FacemarkKazemi_Params()) };
			ret
		}
		
		/// configfile stores the name of the file containing the values of training parameters
		#[inline]
		fn configfile(&self) -> String {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propConfigfile_const(self.as_raw_FacemarkKazemi_Params()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::face::FacemarkKazemi_Params]
	pub trait FacemarkKazemi_ParamsTrait: crate::face::FacemarkKazemi_ParamsTraitConst {
		fn as_raw_mut_FacemarkKazemi_Params(&mut self) -> *mut c_void;
	
		/// cascade_depth This stores the deapth of cascade used for training.
		#[inline]
		fn set_cascade_depth(&mut self, val: u32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propCascade_depth_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// tree_depth This stores the max height of the regression tree built.
		#[inline]
		fn set_tree_depth(&mut self, val: u32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propTree_depth_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// num_trees_per_cascade_level This stores number of trees fit per cascade level.
		#[inline]
		fn set_num_trees_per_cascade_level(&mut self, val: u32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propNum_trees_per_cascade_level_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// learning_rate stores the learning rate in gradient boosting, also referred as shrinkage.
		#[inline]
		fn set_learning_rate(&mut self, val: f32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propLearning_rate_float(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// oversampling_amount stores number of initialisations used to create training samples.
		#[inline]
		fn set_oversampling_amount(&mut self, val: u32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propOversampling_amount_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// num_test_coordinates stores number of test coordinates.
		#[inline]
		fn set_num_test_coordinates(&mut self, val: u32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propNum_test_coordinates_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// lambda stores a value to calculate probability of closeness of two coordinates.
		#[inline]
		fn set_lambda(&mut self, val: f32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propLambda_float(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// num_test_splits stores number of random test splits generated.
		#[inline]
		fn set_num_test_splits(&mut self, val: u32) {
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propNum_test_splits_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) };
			ret
		}
		
		/// configfile stores the name of the file containing the values of training parameters
		#[inline]
		fn set_configfile(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_face_FacemarkKazemi_Params_propConfigfile_String(self.as_raw_mut_FacemarkKazemi_Params(), val.opencv_as_extern_mut()) };
			ret
		}
		
	}
	
	pub struct FacemarkKazemi_Params {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkKazemi_Params }
	
	impl Drop for FacemarkKazemi_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkKazemi_Params_delete(self.as_raw_mut_FacemarkKazemi_Params()) };
		}
	}
	
	unsafe impl Send for FacemarkKazemi_Params {}
	
	impl crate::face::FacemarkKazemi_ParamsTraitConst for FacemarkKazemi_Params {
		#[inline] fn as_raw_FacemarkKazemi_Params(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkKazemi_ParamsTrait for FacemarkKazemi_Params {
		#[inline] fn as_raw_mut_FacemarkKazemi_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkKazemi_Params {
		/// \brief Constructor
		#[inline]
		pub fn default() -> Result<crate::face::FacemarkKazemi_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkKazemi_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::FacemarkKazemi_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for FacemarkKazemi_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkKazemi_Params")
				.field("cascade_depth", &crate::face::FacemarkKazemi_ParamsTraitConst::cascade_depth(self))
				.field("tree_depth", &crate::face::FacemarkKazemi_ParamsTraitConst::tree_depth(self))
				.field("num_trees_per_cascade_level", &crate::face::FacemarkKazemi_ParamsTraitConst::num_trees_per_cascade_level(self))
				.field("learning_rate", &crate::face::FacemarkKazemi_ParamsTraitConst::learning_rate(self))
				.field("oversampling_amount", &crate::face::FacemarkKazemi_ParamsTraitConst::oversampling_amount(self))
				.field("num_test_coordinates", &crate::face::FacemarkKazemi_ParamsTraitConst::num_test_coordinates(self))
				.field("lambda", &crate::face::FacemarkKazemi_ParamsTraitConst::lambda(self))
				.field("num_test_splits", &crate::face::FacemarkKazemi_ParamsTraitConst::num_test_splits(self))
				.field("configfile", &crate::face::FacemarkKazemi_ParamsTraitConst::configfile(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkLBF]
	pub trait FacemarkLBFTraitConst: crate::face::FacemarkTrainTraitConst {
		fn as_raw_FacemarkLBF(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::FacemarkLBF]
	pub trait FacemarkLBFTrait: crate::face::FacemarkLBFTraitConst + crate::face::FacemarkTrainTrait {
		fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void;
	
	}
	
	pub struct FacemarkLBF {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkLBF }
	
	impl Drop for FacemarkLBF {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkLBF_delete(self.as_raw_mut_FacemarkLBF()) };
		}
	}
	
	unsafe impl Send for FacemarkLBF {}
	
	impl core::AlgorithmTraitConst for FacemarkLBF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FacemarkLBF {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for FacemarkLBF {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrait for FacemarkLBF {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrainTraitConst for FacemarkLBF {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrainTrait for FacemarkLBF {
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkLBFTraitConst for FacemarkLBF {
		#[inline] fn as_raw_FacemarkLBF(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkLBFTrait for FacemarkLBF {
		#[inline] fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkLBF {
		/// ## C++ default parameters
		/// * parameters: FacemarkLBF::Params()
		#[inline]
		pub fn create(parameters: &crate::face::FacemarkLBF_Params) -> Result<core::Ptr<crate::face::FacemarkLBF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkLBF_create_const_ParamsR(parameters.as_raw_FacemarkLBF_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FacemarkLBF>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * parameters: FacemarkLBF::Params()
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::FacemarkLBF>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkLBF_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FacemarkLBF>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { FacemarkLBF, core::Algorithm, cv_face_FacemarkLBF_to_Algorithm }
	
	boxed_cast_base! { FacemarkLBF, crate::face::Facemark, cv_face_FacemarkLBF_to_Facemark }
	
	boxed_cast_base! { FacemarkLBF, crate::face::FacemarkTrain, cv_face_FacemarkLBF_to_FacemarkTrain }
	
	impl std::fmt::Debug for FacemarkLBF {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkLBF")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkLBF_Params]
	pub trait FacemarkLBF_ParamsTraitConst {
		fn as_raw_FacemarkLBF_Params(&self) -> *const c_void;
	
		#[inline]
		fn shape_offset(&self) -> f64 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propShape_offset_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn cascade_face(&self) -> String {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propCascade_face_const(self.as_raw_FacemarkLBF_Params()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn verbose(&self) -> bool {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propVerbose_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn n_landmarks(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propN_landmarks_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn init_shape_n(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propInitShape_n_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn stages_n(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propStages_n_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn tree_n(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propTree_n_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn tree_depth(&self) -> i32 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propTree_depth_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn bagging_overlap(&self) -> f64 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propBagging_overlap_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn model_filename(&self) -> String {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propModel_filename_const(self.as_raw_FacemarkLBF_Params()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// flag to save the trained model or not
		#[inline]
		fn save_model(&self) -> bool {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propSave_model_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		/// seed for shuffling the training data
		#[inline]
		fn seed(&self) -> u32 {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propSeed_const(self.as_raw_FacemarkLBF_Params()) };
			ret
		}
		
		#[inline]
		fn feats_m(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propFeats_m_const(self.as_raw_FacemarkLBF_Params()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn radius_m(&self) -> core::Vector<f64> {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propRadius_m_const(self.as_raw_FacemarkLBF_Params()) };
			let ret = unsafe { core::Vector::<f64>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn detect_roi(&self) -> core::Rect {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkLBF_Params_propDetectROI_const(self.as_raw_FacemarkLBF_Params(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkLBF_Params_write_const_FileStorageR(self.as_raw_FacemarkLBF_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::FacemarkLBF_Params]
	pub trait FacemarkLBF_ParamsTrait: crate::face::FacemarkLBF_ParamsTraitConst {
		fn as_raw_mut_FacemarkLBF_Params(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_shape_offset(&mut self, val: f64) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propShape_offset_double(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_cascade_face(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propCascade_face_String(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern_mut()) };
			ret
		}
		
		#[inline]
		fn set_verbose(&mut self, val: bool) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propVerbose_bool(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_n_landmarks(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propN_landmarks_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_init_shape_n(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propInitShape_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_stages_n(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propStages_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_tree_n(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propTree_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_tree_depth(&mut self, val: i32) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propTree_depth_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_bagging_overlap(&mut self, val: f64) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propBagging_overlap_double(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_model_filename(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propModel_filename_string(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern_mut()) };
			ret
		}
		
		/// flag to save the trained model or not
		#[inline]
		fn set_save_model(&mut self, val: bool) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propSave_model_bool(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		/// seed for shuffling the training data
		#[inline]
		fn set_seed(&mut self, val: u32) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propSeed_unsigned_int(self.as_raw_mut_FacemarkLBF_Params(), val) };
			ret
		}
		
		#[inline]
		fn set_feats_m(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propFeats_m_vectorLintG(self.as_raw_mut_FacemarkLBF_Params(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
		#[inline]
		fn set_radius_m(&mut self, mut val: core::Vector<f64>) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propRadius_m_vectorLdoubleG(self.as_raw_mut_FacemarkLBF_Params(), val.as_raw_mut_VectorOff64()) };
			ret
		}
		
		#[inline]
		fn set_detect_roi(&mut self, val: core::Rect) {
			let ret = unsafe { sys::cv_face_FacemarkLBF_Params_propDetectROI_Rect(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkLBF_Params_read_const_FileNodeR(self.as_raw_mut_FacemarkLBF_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct FacemarkLBF_Params {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkLBF_Params }
	
	impl Drop for FacemarkLBF_Params {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkLBF_Params_delete(self.as_raw_mut_FacemarkLBF_Params()) };
		}
	}
	
	unsafe impl Send for FacemarkLBF_Params {}
	
	impl crate::face::FacemarkLBF_ParamsTraitConst for FacemarkLBF_Params {
		#[inline] fn as_raw_FacemarkLBF_Params(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkLBF_ParamsTrait for FacemarkLBF_Params {
		#[inline] fn as_raw_mut_FacemarkLBF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkLBF_Params {
		/// \brief Constructor
		#[inline]
		pub fn default() -> Result<crate::face::FacemarkLBF_Params> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkLBF_Params_Params(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::FacemarkLBF_Params::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for FacemarkLBF_Params {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkLBF_Params")
				.field("shape_offset", &crate::face::FacemarkLBF_ParamsTraitConst::shape_offset(self))
				.field("cascade_face", &crate::face::FacemarkLBF_ParamsTraitConst::cascade_face(self))
				.field("verbose", &crate::face::FacemarkLBF_ParamsTraitConst::verbose(self))
				.field("n_landmarks", &crate::face::FacemarkLBF_ParamsTraitConst::n_landmarks(self))
				.field("init_shape_n", &crate::face::FacemarkLBF_ParamsTraitConst::init_shape_n(self))
				.field("stages_n", &crate::face::FacemarkLBF_ParamsTraitConst::stages_n(self))
				.field("tree_n", &crate::face::FacemarkLBF_ParamsTraitConst::tree_n(self))
				.field("tree_depth", &crate::face::FacemarkLBF_ParamsTraitConst::tree_depth(self))
				.field("bagging_overlap", &crate::face::FacemarkLBF_ParamsTraitConst::bagging_overlap(self))
				.field("model_filename", &crate::face::FacemarkLBF_ParamsTraitConst::model_filename(self))
				.field("save_model", &crate::face::FacemarkLBF_ParamsTraitConst::save_model(self))
				.field("seed", &crate::face::FacemarkLBF_ParamsTraitConst::seed(self))
				.field("feats_m", &crate::face::FacemarkLBF_ParamsTraitConst::feats_m(self))
				.field("radius_m", &crate::face::FacemarkLBF_ParamsTraitConst::radius_m(self))
				.field("detect_roi", &crate::face::FacemarkLBF_ParamsTraitConst::detect_roi(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FacemarkTrain]
	pub trait FacemarkTrainTraitConst: crate::face::FacemarkTraitConst {
		fn as_raw_FacemarkTrain(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::FacemarkTrain]
	pub trait FacemarkTrainTrait: crate::face::FacemarkTrainTraitConst + crate::face::FacemarkTrait {
		fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void;
	
		/// Add one training sample to the trainer.
		/// 
		/// ## Parameters
		/// * image: Input image.
		/// * landmarks: The ground-truth of facial landmarks points corresponds to the image.
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// String imageFiles = "../data/images_train.txt";
		/// String ptsFiles = "../data/points_train.txt";
		/// std::vector<String> images_train;
		/// std::vector<String> landmarks_train;
		/// 
		/// // load the list of dataset: image paths and landmark file paths
		/// loadDatasetList(imageFiles,ptsFiles,images_train,landmarks_train);
		/// 
		/// Mat image;
		/// std::vector<Point2f> facial_points;
		/// for(size_t i=0;i<images_train.size();i++){
		///    image = imread(images_train[i].c_str());
		///    loadFacePoints(landmarks_train[i],facial_points);
		///    facemark->addTrainingSample(image, facial_points);
		/// }
		/// ```
		/// 
		/// 
		/// The contents in the training files should follows the standard format.
		/// Here are examples for the contents in these files.
		/// example of content in the images_train.txt
		/// ```C++
		/// /home/user/ibug/image_003_1.jpg
		/// /home/user/ibug/image_004_1.jpg
		/// /home/user/ibug/image_005_1.jpg
		/// /home/user/ibug/image_006.jpg
		/// ```
		/// 
		/// 
		/// example of content in the points_train.txt
		/// ```C++
		/// /home/user/ibug/image_003_1.pts
		/// /home/user/ibug/image_004_1.pts
		/// /home/user/ibug/image_005_1.pts
		/// /home/user/ibug/image_006.pts
		/// ```
		/// 
		#[inline]
		fn add_training_sample(&mut self, image: &impl core::ToInputArray, landmarks: &impl core::ToInputArray) -> Result<bool> {
			input_array_arg!(image);
			input_array_arg!(landmarks);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkTrain_addTrainingSample_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FacemarkTrain(), image.as_raw__InputArray(), landmarks.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Trains a Facemark algorithm using the given dataset.
		/// Before the training process, training samples should be added to the trainer
		/// using face::addTrainingSample function.
		/// 
		/// ## Parameters
		/// * parameters: Optional extra parameters (algorithm dependent).
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// FacemarkLBF::Params params;
		/// params.model_filename = "ibug68.model"; // filename to save the trained model
		/// Ptr<Facemark> facemark = FacemarkLBF::create(params);
		/// 
		/// // add training samples (see Facemark::addTrainingSample)
		/// 
		/// facemark->training();
		/// ```
		/// 
		/// 
		/// ## C++ default parameters
		/// * parameters: 0
		#[inline]
		unsafe fn training(&mut self, parameters: *mut c_void) -> Result<()> {
			return_send!(via ocvrs_return);
			{ sys::cv_face_FacemarkTrain_training_voidX(self.as_raw_mut_FacemarkTrain(), parameters, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Trains a Facemark algorithm using the given dataset.
		/// Before the training process, training samples should be added to the trainer
		/// using face::addTrainingSample function.
		/// 
		/// ## Parameters
		/// * parameters: Optional extra parameters (algorithm dependent).
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// FacemarkLBF::Params params;
		/// params.model_filename = "ibug68.model"; // filename to save the trained model
		/// Ptr<Facemark> facemark = FacemarkLBF::create(params);
		/// 
		///  add training samples (see Facemark::addTrainingSample)
		/// 
		/// facemark->training();
		/// ```
		/// 
		/// 
		/// ## Note
		/// This alternative version of [training] function uses the following default values for its arguments:
		/// * parameters: 0
		#[inline]
		fn training_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkTrain_training(self.as_raw_mut_FacemarkTrain(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Set a user defined face detector for the Facemark algorithm.
		/// ## Parameters
		/// * detector: The user defined face detector function
		/// * userData: Detector parameters
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// MyDetectorParameters detectorParameters(...);
		/// facemark->setFaceDetector(myDetector, &detectorParameters);
		/// ```
		/// 
		/// 
		/// Example of a user defined face detector
		/// ```C++
		/// bool myDetector( InputArray image, OutputArray faces, void* userData)
		/// {
		///    MyDetectorParameters* params = (MyDetectorParameters*)userData;
		///    // -------- do something --------
		/// }
		/// ```
		/// 
		/// 
		/// TODO Lifetime of detector parameters is uncontrolled. Rework interface design to "Ptr<FaceDetector>".
		/// 
		/// ## C++ default parameters
		/// * user_data: 0
		#[inline]
		fn set_face_detector(&mut self, detector: crate::face::FN_FaceDetector) -> Result<bool> {
			callback_arg!(detector_trampoline(unnamed: *const c_void, unnamed_1: *const c_void, user_data: *mut c_void) -> bool => user_data in callbacks => detector(unnamed: *const c_void, unnamed_1: *const c_void) -> bool);
			userdata_arg!(user_data in callbacks => detector);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(self.as_raw_mut_FacemarkTrain(), detector_trampoline, user_data, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Detect faces from a given image using default or user defined face detector.
		/// Some Algorithm might not provide a default face detector.
		/// 
		/// ## Parameters
		/// * image: Input image.
		/// * faces: Output of the function which represent region of interest of the detected faces. Each face is stored in cv::Rect container.
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// std::vector<cv::Rect> faces;
		/// facemark->getFaces(img, faces);
		/// for(int j=0;j<faces.size();j++){
		///    cv::rectangle(img, faces[j], cv::Scalar(255,0,255));
		/// }
		/// ```
		/// 
		#[inline]
		fn get_faces(&mut self, image: &impl core::ToInputArray, faces: &mut impl core::ToOutputArray) -> Result<bool> {
			input_array_arg!(image);
			output_array_arg!(faces);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkTrain_getFaces_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FacemarkTrain(), image.as_raw__InputArray(), faces.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Get data from an algorithm
		/// 
		/// ## Parameters
		/// * items: The obtained data, algorithm dependent.
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// Ptr<FacemarkAAM> facemark = FacemarkAAM::create();
		/// facemark->loadModel("AAM.yml");
		/// 
		/// FacemarkAAM::Data data;
		/// facemark->getData(&data);
		/// std::vector<Point2f> s0 = data.s0;
		/// 
		/// cout<<s0<<endl;
		/// ```
		/// 
		/// 
		/// ## C++ default parameters
		/// * items: 0
		#[inline]
		unsafe fn get_data(&mut self, items: *mut c_void) -> Result<bool> {
			return_send!(via ocvrs_return);
			{ sys::cv_face_FacemarkTrain_getData_voidX(self.as_raw_mut_FacemarkTrain(), items, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Get data from an algorithm
		/// 
		/// ## Parameters
		/// * items: The obtained data, algorithm dependent.
		/// 
		/// <B>Example of usage</B>
		/// ```C++
		/// Ptr<FacemarkAAM> facemark = FacemarkAAM::create();
		/// facemark->loadModel("AAM.yml");
		/// 
		/// FacemarkAAM::Data data;
		/// facemark->getData(&data);
		/// std::vector<Point2f> s0 = data.s0;
		/// 
		/// cout<<s0<<endl;
		/// ```
		/// 
		/// 
		/// ## Note
		/// This alternative version of [get_data] function uses the following default values for its arguments:
		/// * items: 0
		#[inline]
		fn get_data_def(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FacemarkTrain_getData(self.as_raw_mut_FacemarkTrain(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for trainable facemark models
	/// 
	/// To utilize this API in your program, please take a look at the [tutorial_table_of_content_facemark]
	/// ### Description
	/// 
	/// The AAM and LBF facemark models in OpenCV are derived from the abstract base class FacemarkTrain, which
	/// provides a unified access to those facemark algorithms in OpenCV.
	/// 
	/// Here is an example on how to declare facemark algorithm:
	/// ```C++
	/// // Using Facemark in your code:
	/// Ptr<Facemark> facemark = FacemarkLBF::create();
	/// ```
	/// 
	/// 
	/// 
	/// The typical pipeline for facemark detection is listed as follows:
	/// - (Non-mandatory) Set a user defined face detection using FacemarkTrain::setFaceDetector.
	///   The facemark algorithms are designed to fit the facial points into a face.
	///   Therefore, the face information should be provided to the facemark algorithm.
	///   Some algorithms might provides a default face recognition function.
	///   However, the users might prefer to use their own face detector to obtains the best possible detection result.
	/// - (Non-mandatory) Training the model for a specific algorithm using FacemarkTrain::training.
	///   In this case, the model should be automatically saved by the algorithm.
	///   If the user already have a trained model, then this part can be omitted.
	/// - Load the trained model using Facemark::loadModel.
	/// - Perform the fitting via the Facemark::fit.
	pub struct FacemarkTrain {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FacemarkTrain }
	
	impl Drop for FacemarkTrain {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FacemarkTrain_delete(self.as_raw_mut_FacemarkTrain()) };
		}
	}
	
	unsafe impl Send for FacemarkTrain {}
	
	impl core::AlgorithmTraitConst for FacemarkTrain {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FacemarkTrain {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for FacemarkTrain {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrait for FacemarkTrain {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrainTraitConst for FacemarkTrain {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FacemarkTrainTrait for FacemarkTrain {
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FacemarkTrain {
	}
	
	boxed_cast_descendant! { FacemarkTrain, crate::face::FacemarkAAM, cv_face_FacemarkTrain_to_FacemarkAAM }
	
	boxed_cast_descendant! { FacemarkTrain, crate::face::FacemarkLBF, cv_face_FacemarkTrain_to_FacemarkLBF }
	
	boxed_cast_base! { FacemarkTrain, core::Algorithm, cv_face_FacemarkTrain_to_Algorithm }
	
	boxed_cast_base! { FacemarkTrain, crate::face::Facemark, cv_face_FacemarkTrain_to_Facemark }
	
	impl std::fmt::Debug for FacemarkTrain {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FacemarkTrain")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::FisherFaceRecognizer]
	pub trait FisherFaceRecognizerTraitConst: crate::face::BasicFaceRecognizerTraitConst {
		fn as_raw_FisherFaceRecognizer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::FisherFaceRecognizer]
	pub trait FisherFaceRecognizerTrait: crate::face::BasicFaceRecognizerTrait + crate::face::FisherFaceRecognizerTraitConst {
		fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void;
	
	}
	
	pub struct FisherFaceRecognizer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { FisherFaceRecognizer }
	
	impl Drop for FisherFaceRecognizer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_FisherFaceRecognizer_delete(self.as_raw_mut_FisherFaceRecognizer()) };
		}
	}
	
	unsafe impl Send for FisherFaceRecognizer {}
	
	impl core::AlgorithmTraitConst for FisherFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for FisherFaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizerTraitConst for FisherFaceRecognizer {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::BasicFaceRecognizerTrait for FisherFaceRecognizer {
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for FisherFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for FisherFaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FisherFaceRecognizerTraitConst for FisherFaceRecognizer {
		#[inline] fn as_raw_FisherFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FisherFaceRecognizerTrait for FisherFaceRecognizer {
		#[inline] fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl FisherFaceRecognizer {
		/// ## Parameters
		/// * num_components: The number of components (read: Fisherfaces) kept for this Linear
		/// Discriminant Analysis with the Fisherfaces criterion. It's useful to keep all components, that
		/// means the number of your classes c (read: subjects, persons you want to recognize). If you leave
		/// this at the default (0) or set it to a value less-equal 0 or greater (c-1), it will be set to the
		/// correct number (c-1) automatically.
		/// * threshold: The threshold applied in the prediction. If the distance to the nearest neighbor
		/// is larger than the threshold, this method returns -1.
		/// 
		/// ### Notes:
		/// 
		/// *   Training and prediction must be done on grayscale images, use cvtColor to convert between the
		///    color spaces.
		/// *   **THE FISHERFACES METHOD MAKES THE ASSUMPTION, THAT THE TRAINING AND TEST IMAGES ARE OF EQUAL
		///    SIZE.** (caps-lock, because I got so many mails asking for this). You have to make sure your
		///    input data has the correct shape, else a meaningful exception is thrown. Use resize to resize
		///    the images.
		/// *   This model does not support updating.
		/// 
		/// ### Model internal data:
		/// 
		/// *   num_components see FisherFaceRecognizer::create.
		/// *   threshold see FisherFaceRecognizer::create.
		/// *   eigenvalues The eigenvalues for this Linear Discriminant Analysis (ordered descending).
		/// *   eigenvectors The eigenvectors for this Linear Discriminant Analysis (ordered by their
		///    eigenvalue).
		/// *   mean The sample mean calculated from the training data.
		/// *   projections The projections of the training data.
		/// *   labels The labels corresponding to the projections.
		/// 
		/// ## C++ default parameters
		/// * num_components: 0
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create(num_components: i32, threshold: f64) -> Result<core::Ptr<crate::face::FisherFaceRecognizer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FisherFaceRecognizer_create_int_double(num_components, threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FisherFaceRecognizer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Parameters
		/// * num_components: The number of components (read: Fisherfaces) kept for this Linear
		/// Discriminant Analysis with the Fisherfaces criterion. It's useful to keep all components, that
		/// means the number of your classes c (read: subjects, persons you want to recognize). If you leave
		/// this at the default (0) or set it to a value less-equal 0 or greater (c-1), it will be set to the
		/// correct number (c-1) automatically.
		/// * threshold: The threshold applied in the prediction. If the distance to the nearest neighbor
		/// is larger than the threshold, this method returns -1.
		/// 
		/// ### Notes:
		/// 
		/// *   Training and prediction must be done on grayscale images, use cvtColor to convert between the
		///    color spaces.
		/// *   **THE FISHERFACES METHOD MAKES THE ASSUMPTION, THAT THE TRAINING AND TEST IMAGES ARE OF EQUAL
		///    SIZE.** (caps-lock, because I got so many mails asking for this). You have to make sure your
		///    input data has the correct shape, else a meaningful exception is thrown. Use resize to resize
		///    the images.
		/// *   This model does not support updating.
		/// 
		/// ### Model internal data:
		/// 
		/// *   num_components see FisherFaceRecognizer::create.
		/// *   threshold see FisherFaceRecognizer::create.
		/// *   eigenvalues The eigenvalues for this Linear Discriminant Analysis (ordered descending).
		/// *   eigenvectors The eigenvectors for this Linear Discriminant Analysis (ordered by their
		///    eigenvalue).
		/// *   mean The sample mean calculated from the training data.
		/// *   projections The projections of the training data.
		/// *   labels The labels corresponding to the projections.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * num_components: 0
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::FisherFaceRecognizer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_FisherFaceRecognizer_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::FisherFaceRecognizer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { FisherFaceRecognizer, core::Algorithm, cv_face_FisherFaceRecognizer_to_Algorithm }
	
	boxed_cast_base! { FisherFaceRecognizer, crate::face::BasicFaceRecognizer, cv_face_FisherFaceRecognizer_to_BasicFaceRecognizer }
	
	boxed_cast_base! { FisherFaceRecognizer, crate::face::FaceRecognizer, cv_face_FisherFaceRecognizer_to_FaceRecognizer }
	
	impl std::fmt::Debug for FisherFaceRecognizer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("FisherFaceRecognizer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::LBPHFaceRecognizer]
	pub trait LBPHFaceRecognizerTraitConst: crate::face::FaceRecognizerTraitConst {
		fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void;
	
		/// ## See also
		/// setGridX
		#[inline]
		fn get_grid_x(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_getGridX_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setGridY
		#[inline]
		fn get_grid_y(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_getGridY_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setRadius
		#[inline]
		fn get_radius(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_getRadius_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setNeighbors
		#[inline]
		fn get_neighbors(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_getNeighbors_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setThreshold
		#[inline]
		fn get_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_getThreshold_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_histograms(&self) -> Result<core::Vector<core::Mat>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_getHistograms_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_labels(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_getLabels_const(self.as_raw_LBPHFaceRecognizer(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::LBPHFaceRecognizer]
	pub trait LBPHFaceRecognizerTrait: crate::face::FaceRecognizerTrait + crate::face::LBPHFaceRecognizerTraitConst {
		fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void;
	
		/// ## See also
		/// setGridX getGridX
		#[inline]
		fn set_grid_x(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_setGridX_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setGridY getGridY
		#[inline]
		fn set_grid_y(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_setGridY_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setRadius getRadius
		#[inline]
		fn set_radius(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_setRadius_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setNeighbors getNeighbors
		#[inline]
		fn set_neighbors(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_setNeighbors_int(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## See also
		/// setThreshold getThreshold
		#[inline]
		fn set_threshold(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_setThreshold_double(self.as_raw_mut_LBPHFaceRecognizer(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct LBPHFaceRecognizer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { LBPHFaceRecognizer }
	
	impl Drop for LBPHFaceRecognizer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_LBPHFaceRecognizer_delete(self.as_raw_mut_LBPHFaceRecognizer()) };
		}
	}
	
	unsafe impl Send for LBPHFaceRecognizer {}
	
	impl core::AlgorithmTraitConst for LBPHFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for LBPHFaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for LBPHFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for LBPHFaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::LBPHFaceRecognizerTraitConst for LBPHFaceRecognizer {
		#[inline] fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::LBPHFaceRecognizerTrait for LBPHFaceRecognizer {
		#[inline] fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LBPHFaceRecognizer {
		/// ## Parameters
		/// * radius: The radius used for building the Circular Local Binary Pattern. The greater the
		/// radius, the smoother the image but more spatial information you can get.
		/// * neighbors: The number of sample points to build a Circular Local Binary Pattern from. An
		/// appropriate value is to use `8` sample points. Keep in mind: the more sample points you include,
		/// the higher the computational cost.
		/// * grid_x: The number of cells in the horizontal direction, 8 is a common value used in
		/// publications. The more cells, the finer the grid, the higher the dimensionality of the resulting
		/// feature vector.
		/// * grid_y: The number of cells in the vertical direction, 8 is a common value used in
		/// publications. The more cells, the finer the grid, the higher the dimensionality of the resulting
		/// feature vector.
		/// * threshold: The threshold applied in the prediction. If the distance to the nearest neighbor
		/// is larger than the threshold, this method returns -1.
		/// 
		/// ### Notes:
		/// 
		/// *   The Circular Local Binary Patterns (used in training and prediction) expect the data given as
		///    grayscale images, use cvtColor to convert between the color spaces.
		/// *   This model supports updating.
		/// 
		/// ### Model internal data:
		/// 
		/// *   radius see LBPHFaceRecognizer::create.
		/// *   neighbors see LBPHFaceRecognizer::create.
		/// *   grid_x see LLBPHFaceRecognizer::create.
		/// *   grid_y see LBPHFaceRecognizer::create.
		/// *   threshold see LBPHFaceRecognizer::create.
		/// *   histograms Local Binary Patterns Histograms calculated from the given training data (empty if
		///    none was given).
		/// *   labels Labels corresponding to the calculated Local Binary Patterns Histograms.
		/// 
		/// ## C++ default parameters
		/// * radius: 1
		/// * neighbors: 8
		/// * grid_x: 8
		/// * grid_y: 8
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create(radius: i32, neighbors: i32, grid_x: i32, grid_y: i32, threshold: f64) -> Result<core::Ptr<crate::face::LBPHFaceRecognizer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(radius, neighbors, grid_x, grid_y, threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::LBPHFaceRecognizer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Parameters
		/// * radius: The radius used for building the Circular Local Binary Pattern. The greater the
		/// radius, the smoother the image but more spatial information you can get.
		/// * neighbors: The number of sample points to build a Circular Local Binary Pattern from. An
		/// appropriate value is to use `8` sample points. Keep in mind: the more sample points you include,
		/// the higher the computational cost.
		/// * grid_x: The number of cells in the horizontal direction, 8 is a common value used in
		/// publications. The more cells, the finer the grid, the higher the dimensionality of the resulting
		/// feature vector.
		/// * grid_y: The number of cells in the vertical direction, 8 is a common value used in
		/// publications. The more cells, the finer the grid, the higher the dimensionality of the resulting
		/// feature vector.
		/// * threshold: The threshold applied in the prediction. If the distance to the nearest neighbor
		/// is larger than the threshold, this method returns -1.
		/// 
		/// ### Notes:
		/// 
		/// *   The Circular Local Binary Patterns (used in training and prediction) expect the data given as
		///    grayscale images, use cvtColor to convert between the color spaces.
		/// *   This model supports updating.
		/// 
		/// ### Model internal data:
		/// 
		/// *   radius see LBPHFaceRecognizer::create.
		/// *   neighbors see LBPHFaceRecognizer::create.
		/// *   grid_x see LLBPHFaceRecognizer::create.
		/// *   grid_y see LBPHFaceRecognizer::create.
		/// *   threshold see LBPHFaceRecognizer::create.
		/// *   histograms Local Binary Patterns Histograms calculated from the given training data (empty if
		///    none was given).
		/// *   labels Labels corresponding to the calculated Local Binary Patterns Histograms.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * radius: 1
		/// * neighbors: 8
		/// * grid_x: 8
		/// * grid_y: 8
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::LBPHFaceRecognizer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_LBPHFaceRecognizer_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::LBPHFaceRecognizer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { LBPHFaceRecognizer, core::Algorithm, cv_face_LBPHFaceRecognizer_to_Algorithm }
	
	boxed_cast_base! { LBPHFaceRecognizer, crate::face::FaceRecognizer, cv_face_LBPHFaceRecognizer_to_FaceRecognizer }
	
	impl std::fmt::Debug for LBPHFaceRecognizer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LBPHFaceRecognizer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::MACE]
	pub trait MACETraitConst: core::AlgorithmTraitConst {
		fn as_raw_MACE(&self) -> *const c_void;
	
		/// correlate query img and threshold to min class value
		/// ## Parameters
		/// * query: a Mat with query image
		#[inline]
		fn same(&self, query: &impl core::ToInputArray) -> Result<bool> {
			input_array_arg!(query);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_MACE_same_const_const__InputArrayR(self.as_raw_MACE(), query.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::MACE]
	pub trait MACETrait: core::AlgorithmTrait + crate::face::MACETraitConst {
		fn as_raw_mut_MACE(&mut self) -> *mut c_void;
	
		/// optionally encrypt images with random convolution
		/// ## Parameters
		/// * passphrase: a crc64 random seed will get generated from this
		#[inline]
		fn salt(&mut self, passphrase: &str) -> Result<()> {
			extern_container_arg!(passphrase);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_MACE_salt_const_StringR(self.as_raw_mut_MACE(), passphrase.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// train it on positive features
		///    compute the mace filter: `h = D(-1) * X * (X(+) * D(-1) * X)(-1) * C`
		///    also calculate a minimal threshold for this class, the smallest self-similarity from the train images
		/// ## Parameters
		/// * images: a vector<Mat> with the train images
		#[inline]
		fn train(&mut self, images: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(images);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_MACE_train_const__InputArrayR(self.as_raw_mut_MACE(), images.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Minimum Average Correlation Energy Filter
	///    useful for authentication with (cancellable) biometrical features.
	///    (does not need many positives to train (10-50), and no negatives at all, also robust to noise/salting)
	/// 
	///    see also: [Savvides04](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Savvides04)
	/// 
	///    this implementation is largely based on: <https://code.google.com/archive/p/pam-face-authentication> (GSOC 2009)
	/// 
	///    use it like:
	///    ```C++
	/// 
	///    Ptr<face::MACE> mace = face::MACE::create(64);
	/// 
	///    vector<Mat> pos_images = ...
	///    mace->train(pos_images);
	/// 
	///    Mat query = ...
	///    bool same = mace->same(query);
	/// 
	///    ```
	/// 
	/// 
	///    you can also use two-factor authentication, with an additional passphrase:
	/// 
	///    ```C++
	///    String owners_passphrase = "ilikehotdogs";
	///    Ptr<face::MACE> mace = face::MACE::create(64);
	///    mace->salt(owners_passphrase);
	///    vector<Mat> pos_images = ...
	///    mace->train(pos_images);
	/// 
	///    // now, users have to give a valid passphrase, along with the image:
	///    Mat query = ...
	///    cout << "enter passphrase: ";
	///    string pass;
	///    getline(cin, pass);
	///    mace->salt(pass);
	///    bool same = mace->same(query);
	///    ```
	/// 
	/// 
	///    save/load your model:
	///    ```C++
	///    Ptr<face::MACE> mace = face::MACE::create(64);
	///    mace->train(pos_images);
	///    mace->save("my_mace.xml");
	/// 
	///    // later:
	///    Ptr<MACE> reloaded = MACE::load("my_mace.xml");
	///    reloaded->same(some_image);
	///    ```
	/// 
	pub struct MACE {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { MACE }
	
	impl Drop for MACE {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_MACE_delete(self.as_raw_mut_MACE()) };
		}
	}
	
	unsafe impl Send for MACE {}
	
	impl core::AlgorithmTraitConst for MACE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for MACE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::MACETraitConst for MACE {
		#[inline] fn as_raw_MACE(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::MACETrait for MACE {
		#[inline] fn as_raw_mut_MACE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MACE {
		/// constructor
		/// ## Parameters
		/// * filename: build a new MACE instance from a pre-serialized FileStorage
		/// * objname: (optional) top-level node in the FileStorage
		/// 
		/// ## C++ default parameters
		/// * objname: String()
		#[inline]
		pub fn load(filename: &str, objname: &str) -> Result<core::Ptr<crate::face::MACE>> {
			extern_container_arg!(filename);
			extern_container_arg!(objname);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_MACE_load_const_StringR_const_StringR(filename.opencv_as_extern(), objname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::MACE>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// constructor
		/// ## Parameters
		/// * filename: build a new MACE instance from a pre-serialized FileStorage
		/// * objname: (optional) top-level node in the FileStorage
		/// 
		/// ## Note
		/// This alternative version of [load] function uses the following default values for its arguments:
		/// * objname: String()
		#[inline]
		pub fn load_def(filename: &str) -> Result<core::Ptr<crate::face::MACE>> {
			extern_container_arg!(filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_MACE_load_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::MACE>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// constructor
		/// ## Parameters
		/// * IMGSIZE: images will get resized to this (should be an even number)
		/// 
		/// ## C++ default parameters
		/// * imgsize: 64
		#[inline]
		pub fn create(imgsize: i32) -> Result<core::Ptr<crate::face::MACE>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_MACE_create_int(imgsize, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::MACE>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// constructor
		/// ## Parameters
		/// * IMGSIZE: images will get resized to this (should be an even number)
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * imgsize: 64
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::MACE>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_MACE_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::MACE>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { MACE, core::Algorithm, cv_face_MACE_to_Algorithm }
	
	impl std::fmt::Debug for MACE {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("MACE")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::PredictCollector]
	pub trait PredictCollectorTraitConst {
		fn as_raw_PredictCollector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::face::PredictCollector]
	pub trait PredictCollectorTrait: crate::face::PredictCollectorTraitConst {
		fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void;
	
		/// Interface method called by face recognizer before results processing
		/// ## Parameters
		/// * size: total size of prediction evaluation that recognizer could perform
		#[inline]
		fn init(&mut self, size: size_t) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_PredictCollector_init_size_t(self.as_raw_mut_PredictCollector(), size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Interface method called by face recognizer for each result
		/// ## Parameters
		/// * label: current prediction label
		/// * dist: current prediction distance (confidence)
		#[inline]
		fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_PredictCollector_collect_int_double(self.as_raw_mut_PredictCollector(), label, dist, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for all strategies of prediction result handling
	pub struct PredictCollector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { PredictCollector }
	
	impl Drop for PredictCollector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_PredictCollector_delete(self.as_raw_mut_PredictCollector()) };
		}
	}
	
	unsafe impl Send for PredictCollector {}
	
	impl crate::face::PredictCollectorTraitConst for PredictCollector {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::PredictCollectorTrait for PredictCollector {
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PredictCollector {
	}
	
	boxed_cast_descendant! { PredictCollector, crate::face::StandardCollector, cv_face_PredictCollector_to_StandardCollector }
	
	impl std::fmt::Debug for PredictCollector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PredictCollector")
				.finish()
		}
	}
	
	/// Constant methods for [crate::face::StandardCollector]
	pub trait StandardCollectorTraitConst: crate::face::PredictCollectorTraitConst {
		fn as_raw_StandardCollector(&self) -> *const c_void;
	
		/// Returns label with minimal distance
		#[inline]
		fn get_min_label(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_getMinLabel_const(self.as_raw_StandardCollector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns minimal distance value
		#[inline]
		fn get_min_dist(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_getMinDist_const(self.as_raw_StandardCollector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Return results as vector
		/// ## Parameters
		/// * sorted: If set, results will be sorted by distance
		/// Each values is a pair of label and distance.
		/// 
		/// ## C++ default parameters
		/// * sorted: false
		#[inline]
		fn get_results(&self, sorted: bool) -> Result<core::Vector<core::Tuple<(i32, f64)>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_getResults_const_bool(self.as_raw_StandardCollector(), sorted, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Tuple<(i32, f64)>>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Return results as vector
		/// ## Parameters
		/// * sorted: If set, results will be sorted by distance
		/// Each values is a pair of label and distance.
		/// 
		/// ## Note
		/// This alternative version of [get_results] function uses the following default values for its arguments:
		/// * sorted: false
		#[inline]
		fn get_results_def(&self) -> Result<core::Vector<core::Tuple<(i32, f64)>>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_getResults_const(self.as_raw_StandardCollector(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<core::Tuple<(i32, f64)>>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::face::StandardCollector]
	pub trait StandardCollectorTrait: crate::face::PredictCollectorTrait + crate::face::StandardCollectorTraitConst {
		fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void;
	
		/// overloaded interface method
		#[inline]
		fn init(&mut self, size: size_t) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_init_size_t(self.as_raw_mut_StandardCollector(), size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// overloaded interface method
		#[inline]
		fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_collect_int_double(self.as_raw_mut_StandardCollector(), label, dist, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Default predict collector
	/// 
	/// Trace minimal distance with treshhold checking (that is default behavior for most predict logic)
	pub struct StandardCollector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { StandardCollector }
	
	impl Drop for StandardCollector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_face_StandardCollector_delete(self.as_raw_mut_StandardCollector()) };
		}
	}
	
	unsafe impl Send for StandardCollector {}
	
	impl crate::face::PredictCollectorTraitConst for StandardCollector {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::PredictCollectorTrait for StandardCollector {
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::StandardCollectorTraitConst for StandardCollector {
		#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::face::StandardCollectorTrait for StandardCollector {
		#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl StandardCollector {
		/// Constructor
		/// ## Parameters
		/// * threshold_: set threshold
		/// 
		/// ## C++ default parameters
		/// * threshold_: DBL_MAX
		#[inline]
		pub fn new(threshold_: f64) -> Result<crate::face::StandardCollector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_StandardCollector_double(threshold_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::StandardCollector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructor
		/// ## Parameters
		/// * threshold_: set threshold
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * threshold_: DBL_MAX
		#[inline]
		pub fn new_def() -> Result<crate::face::StandardCollector> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_StandardCollector(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::face::StandardCollector::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Static constructor
		/// ## Parameters
		/// * threshold: set threshold
		/// 
		/// ## C++ default parameters
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create(threshold: f64) -> Result<core::Ptr<crate::face::StandardCollector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_create_double(threshold, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::StandardCollector>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Static constructor
		/// ## Parameters
		/// * threshold: set threshold
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * threshold: DBL_MAX
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::face::StandardCollector>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::face::StandardCollector>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { StandardCollector, crate::face::PredictCollector, cv_face_StandardCollector_to_PredictCollector }
	
	impl std::fmt::Debug for StandardCollector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("StandardCollector")
				.finish()
		}
	}
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct StandardCollector_PredictResult {
		pub label: i32,
		pub distance: f64,
	}
	
	opencv_type_simple! { crate::face::StandardCollector_PredictResult }
	
	impl StandardCollector_PredictResult {
		/// ## C++ default parameters
		/// * label_: -1
		/// * distance_: DBL_MAX
		#[inline]
		pub fn new(label_: i32, distance_: f64) -> Result<crate::face::StandardCollector_PredictResult> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_PredictResult_PredictResult_int_double(label_, distance_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * label_: -1
		/// * distance_: DBL_MAX
		#[inline]
		pub fn new_def() -> Result<crate::face::StandardCollector_PredictResult> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_face_StandardCollector_PredictResult_PredictResult(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
}

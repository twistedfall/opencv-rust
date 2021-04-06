#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Face Analysis
//! 
//! - @ref face_changelog
//! - @ref tutorial_face_main
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::PredictCollector, super::StandardCollectorTrait, super::FaceRecognizer, super::BasicFaceRecognizer, super::EigenFaceRecognizer, super::FisherFaceRecognizer, super::LBPHFaceRecognizer, super::Facemark, super::CParamsTrait, super::FacemarkTrain, super::FacemarkLBF_ParamsTrait, super::FacemarkLBF, super::FacemarkAAM_ParamsTrait, super::FacemarkAAM_ConfigTrait, super::FacemarkAAM_DataTrait, super::FacemarkAAM_Model_TextureTrait, super::FacemarkAAM_ModelTrait, super::FacemarkAAM, super::FacemarkKazemi_ParamsTrait, super::FacemarkKazemi, super::MACE, super::BIF };
}

pub type FN_FaceDetector = Option<Box<dyn FnMut(*const c_void, *const c_void) -> bool + Send + Sync + 'static>>;
/// construct an AAM facemark detector
pub fn create_facemark_aam() -> Result<core::Ptr::<dyn crate::face::Facemark>> {
	unsafe { sys::cv_face_createFacemarkAAM() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::Facemark>::opencv_from_extern(r) } )
}

/// construct a Kazemi facemark detector
pub fn create_facemark_kazemi() -> Result<core::Ptr::<dyn crate::face::Facemark>> {
	unsafe { sys::cv_face_createFacemarkKazemi() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::Facemark>::opencv_from_extern(r) } )
}

/// construct an LBF facemark detector
pub fn create_facemark_lbf() -> Result<core::Ptr::<dyn crate::face::Facemark>> {
	unsafe { sys::cv_face_createFacemarkLBF() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::Facemark>::opencv_from_extern(r) } )
}

/// Utility to draw the detected facial landmark points
/// 
/// ## Parameters
/// * image: The input image to be processed.
/// * points: Contains the data of points which will be drawn.
/// * color: The color of points in BGR format represented by cv::Scalar.
/// 
/// <B>Example of usage</B>
/// ```ignore
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
pub fn draw_facemarks(image: &mut dyn core::ToInputOutputArray, points: &dyn core::ToInputArray, color: core::Scalar) -> Result<()> {
	input_output_array_arg!(image);
	input_array_arg!(points);
	unsafe { sys::cv_face_drawFacemarks_const__InputOutputArrayR_const__InputArrayR_Scalar(image.as_raw__InputOutputArray(), points.as_raw__InputArray(), color.opencv_as_extern()) }.into_result()
}

pub fn get_faces_haar(image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray, face_cascade_name: &str) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(faces);
	extern_container_arg!(face_cascade_name);
	unsafe { sys::cv_face_getFacesHAAR_const__InputArrayR_const__OutputArrayR_const_StringR(image.as_raw__InputArray(), faces.as_raw__OutputArray(), face_cascade_name.opencv_as_extern()) }.into_result()
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
/// ```ignore
/// std::vector<cv::Rect> faces;
/// CParams params("haarcascade_frontalface_alt.xml");
/// cv::face::getFaces(frame, faces, &params);
/// for(int j=0;j<faces.size();j++){
///    cv::rectangle(frame, faces[j], cv::Scalar(255,0,255));
/// }
/// cv::imshow("detection", frame);
/// ```
/// 
pub fn get_faces(image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray, params: &mut crate::face::CParams) -> Result<bool> {
	input_array_arg!(image);
	output_array_arg!(faces);
	unsafe { sys::cv_face_getFaces_const__InputArrayR_const__OutputArrayR_CParamsX(image.as_raw__InputArray(), faces.as_raw__OutputArray(), params.as_raw_mut_CParams()) }.into_result()
}

/// A utility to load list of paths to training image and annotation file.
/// ## Parameters
/// * imageList: The specified file contains paths to the training images.
/// * annotationList: The specified file contains paths to the training annotations.
/// * images: The loaded paths of training images.
/// * annotations: The loaded paths of annotation files.
/// 
/// Example of usage:
/// ```ignore
/// String imageFiles = "images_path.txt";
/// String ptsFiles = "annotations_path.txt";
/// std::vector<String> images_train;
/// std::vector<String> landmarks_train;
/// loadDatasetList(imageFiles,ptsFiles,images_train,landmarks_train);
/// ```
/// 
pub fn load_dataset_list(image_list: &str, annotation_list: &str, images: &mut core::Vector::<String>, annotations: &mut core::Vector::<String>) -> Result<bool> {
	extern_container_arg!(mut image_list);
	extern_container_arg!(mut annotation_list);
	unsafe { sys::cv_face_loadDatasetList_String_String_vector_String_R_vector_String_R(image_list.opencv_as_extern_mut(), annotation_list.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), annotations.as_raw_mut_VectorOfString()) }.into_result()
}

/// A utility to load facial landmark information from a given file.
/// 
/// ## Parameters
/// * filename: The filename of file contains the facial landmarks data.
/// * points: The loaded facial landmark points.
/// * offset: An offset value to adjust the loaded points.
/// 
/// <B>Example of usage</B>
/// ```ignore
/// std::vector<Point2f> points;
/// face::loadFacePoints("filename.txt", points, 0.0f);
/// ```
/// 
/// 
/// The annotation file should follow the default format which is
/// ```ignore
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
pub fn load_face_points(filename: &str, points: &mut dyn core::ToOutputArray, offset: f32) -> Result<bool> {
	extern_container_arg!(mut filename);
	output_array_arg!(points);
	unsafe { sys::cv_face_loadFacePoints_String_const__OutputArrayR_float(filename.opencv_as_extern_mut(), points.as_raw__OutputArray(), offset) }.into_result()
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
/// ```ignore
/// cv::String imageFiles = "../data/images_train.txt";
/// cv::String ptsFiles = "../data/points_train.txt";
/// std::vector<String> images;
/// std::vector<std::vector<Point2f> > facePoints;
/// loadTrainingData(imageFiles, ptsFiles, images, facePoints, 0.0f);
/// ```
/// 
/// 
/// example of content in the images_train.txt
/// ```ignore
/// /home/user/ibug/image_003_1.jpg
/// /home/user/ibug/image_004_1.jpg
/// /home/user/ibug/image_005_1.jpg
/// /home/user/ibug/image_006.jpg
/// ```
/// 
/// 
/// example of content in the points_train.txt
/// ```ignore
/// /home/user/ibug/image_003_1.pts
/// /home/user/ibug/image_004_1.pts
/// /home/user/ibug/image_005_1.pts
/// /home/user/ibug/image_006.pts
/// ```
/// 
/// 
/// ## C++ default parameters
/// * offset: 0.0f
pub fn load_training_data_1(image_list: &str, ground_truth: &str, images: &mut core::Vector::<String>, face_points: &mut dyn core::ToOutputArray, offset: f32) -> Result<bool> {
	extern_container_arg!(mut image_list);
	extern_container_arg!(mut ground_truth);
	output_array_arg!(face_points);
	unsafe { sys::cv_face_loadTrainingData_String_String_vector_String_R_const__OutputArrayR_float(image_list.opencv_as_extern_mut(), ground_truth.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), offset) }.into_result()
}

/// A utility to load facial landmark dataset from a single file.
/// 
/// ## Parameters
/// * filename: The filename of a file that contains the dataset information.
/// Each line contains the filename of an image followed by
/// pairs of x and y values of facial landmarks points separated by a space.
/// Example
/// ```ignore
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
/// ```ignore
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
pub fn load_training_data(filename: &str, images: &mut core::Vector::<String>, face_points: &mut dyn core::ToOutputArray, delim: i8, offset: f32) -> Result<bool> {
	extern_container_arg!(mut filename);
	output_array_arg!(face_points);
	unsafe { sys::cv_face_loadTrainingData_String_vector_String_R_const__OutputArrayR_char_float(filename.opencv_as_extern_mut(), images.as_raw_mut_VectorOfString(), face_points.as_raw__OutputArray(), delim, offset) }.into_result()
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
pub fn load_training_data_2(mut filename: core::Vector::<String>, trainlandmarks: &mut core::Vector::<core::Vector::<core::Point2f>>, trainimages: &mut core::Vector::<String>) -> Result<bool> {
	unsafe { sys::cv_face_loadTrainingData_vector_String__vector_vector_Point2f__R_vector_String_R(filename.as_raw_mut_VectorOfString(), trainlandmarks.as_raw_mut_VectorOfVectorOfPoint2f(), trainimages.as_raw_mut_VectorOfString()) }.into_result()
}

/// Implementation of bio-inspired features (BIF) from the paper:
/// Guo, Guodong, et al. "Human age estimation using bio-inspired features."
/// Computer Vision and Pattern Recognition, 2009. CVPR 2009.
pub trait BIF: core::AlgorithmTrait {
	fn as_raw_BIF(&self) -> *const c_void;
	fn as_raw_mut_BIF(&mut self) -> *mut c_void;

	/// ## Returns
	/// The number of filter bands used for computing BIF.
	fn get_num_bands(&self) -> Result<i32> {
		unsafe { sys::cv_face_BIF_getNumBands_const(self.as_raw_BIF()) }.into_result()
	}
	
	/// ## Returns
	/// The number of image rotations.
	fn get_num_rotations(&self) -> Result<i32> {
		unsafe { sys::cv_face_BIF_getNumRotations_const(self.as_raw_BIF()) }.into_result()
	}
	
	/// Computes features sby input image.
	/// ## Parameters
	/// * image: Input image (CV_32FC1).
	/// * features: Feature vector (CV_32FC1).
	fn compute(&self, image: &dyn core::ToInputArray, features: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(features);
		unsafe { sys::cv_face_BIF_compute_const_const__InputArrayR_const__OutputArrayR(self.as_raw_BIF(), image.as_raw__InputArray(), features.as_raw__OutputArray()) }.into_result()
	}
	
}

impl dyn BIF + '_ {
	/// ## Parameters
	/// * num_bands: The number of filter bands (<=8) used for computing BIF.
	/// * num_rotations: The number of image rotations for computing BIF.
	/// ## Returns
	/// Object for computing BIF.
	/// 
	/// ## C++ default parameters
	/// * num_bands: 8
	/// * num_rotations: 12
	pub fn create(num_bands: i32, num_rotations: i32) -> Result<core::Ptr::<dyn crate::face::BIF>> {
		unsafe { sys::cv_face_BIF_create_int_int(num_bands, num_rotations) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::BIF>::opencv_from_extern(r) } )
	}
	
}
pub trait BasicFaceRecognizer: crate::face::FaceRecognizer {
	fn as_raw_BasicFaceRecognizer(&self) -> *const c_void;
	fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void;

	/// ## See also
	/// setNumComponents
	fn get_num_components(&self) -> Result<i32> {
		unsafe { sys::cv_face_BasicFaceRecognizer_getNumComponents_const(self.as_raw_BasicFaceRecognizer()) }.into_result()
	}
	
	/// ## See also
	/// setNumComponents getNumComponents
	fn set_num_components(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_face_BasicFaceRecognizer_setNumComponents_int(self.as_raw_mut_BasicFaceRecognizer(), val) }.into_result()
	}
	
	/// ## See also
	/// setThreshold
	fn get_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_face_BasicFaceRecognizer_getThreshold_const(self.as_raw_BasicFaceRecognizer()) }.into_result()
	}
	
	/// ## See also
	/// setThreshold getThreshold
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_face_BasicFaceRecognizer_setThreshold_double(self.as_raw_mut_BasicFaceRecognizer(), val) }.into_result()
	}
	
	fn get_projections(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_face_BasicFaceRecognizer_getProjections_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn get_labels(&self) -> Result<core::Mat> {
		unsafe { sys::cv_face_BasicFaceRecognizer_getLabels_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn get_eigen_values(&self) -> Result<core::Mat> {
		unsafe { sys::cv_face_BasicFaceRecognizer_getEigenValues_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn get_eigen_vectors(&self) -> Result<core::Mat> {
		unsafe { sys::cv_face_BasicFaceRecognizer_getEigenVectors_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn get_mean(&self) -> Result<core::Mat> {
		unsafe { sys::cv_face_BasicFaceRecognizer_getMean_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_face_BasicFaceRecognizer_read_const_FileNodeR(self.as_raw_mut_BasicFaceRecognizer(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_face_BasicFaceRecognizer_write_const_FileStorageR(self.as_raw_BasicFaceRecognizer(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_face_BasicFaceRecognizer_empty_const(self.as_raw_BasicFaceRecognizer()) }.into_result()
	}
	
}

pub trait CParamsTrait {
	fn as_raw_CParams(&self) -> *const c_void;
	fn as_raw_mut_CParams(&mut self) -> *mut c_void;

	/// the face detector
	fn cascade(&self) -> String {
		unsafe { sys::cv_face_CParams_getPropCascade_const(self.as_raw_CParams()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: cascade")
	}
	
	/// the face detector
	fn set_cascade(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_face_CParams_setPropCascade_String(self.as_raw_mut_CParams(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_cascade")
	}
	
	/// Parameter specifying how much the image size is reduced at each image scale.
	fn scale_factor(&self) -> f64 {
		unsafe { sys::cv_face_CParams_getPropScaleFactor_const(self.as_raw_CParams()) }.into_result().expect("Infallible function failed: scale_factor")
	}
	
	/// Parameter specifying how much the image size is reduced at each image scale.
	fn set_scale_factor(&mut self, val: f64) -> () {
		unsafe { sys::cv_face_CParams_setPropScaleFactor_double(self.as_raw_mut_CParams(), val) }.into_result().expect("Infallible function failed: set_scale_factor")
	}
	
	/// Parameter specifying how many neighbors each candidate rectangle should have to retain it.
	fn min_neighbors(&self) -> i32 {
		unsafe { sys::cv_face_CParams_getPropMinNeighbors_const(self.as_raw_CParams()) }.into_result().expect("Infallible function failed: min_neighbors")
	}
	
	/// Parameter specifying how many neighbors each candidate rectangle should have to retain it.
	fn set_min_neighbors(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_CParams_setPropMinNeighbors_int(self.as_raw_mut_CParams(), val) }.into_result().expect("Infallible function failed: set_min_neighbors")
	}
	
	/// Minimum possible object size.
	fn min_size(&self) -> core::Size {
		unsafe { sys::cv_face_CParams_getPropMinSize_const(self.as_raw_CParams()) }.into_result().expect("Infallible function failed: min_size")
	}
	
	/// Minimum possible object size.
	fn set_min_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_face_CParams_setPropMinSize_Size(self.as_raw_mut_CParams(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_min_size")
	}
	
	/// Maximum possible object size.
	fn max_size(&self) -> core::Size {
		unsafe { sys::cv_face_CParams_getPropMaxSize_const(self.as_raw_CParams()) }.into_result().expect("Infallible function failed: max_size")
	}
	
	/// Maximum possible object size.
	fn set_max_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_face_CParams_setPropMaxSize_Size(self.as_raw_mut_CParams(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_max_size")
	}
	
	fn face_cascade(&mut self) -> crate::objdetect::CascadeClassifier {
		unsafe { sys::cv_face_CParams_getPropFace_cascade(self.as_raw_mut_CParams()) }.into_result().map(|r| unsafe { crate::objdetect::CascadeClassifier::opencv_from_extern(r) } ).expect("Infallible function failed: face_cascade")
	}
	
	fn set_face_cascade(&mut self, mut val: crate::objdetect::CascadeClassifier) -> () {
		unsafe { sys::cv_face_CParams_setPropFace_cascade_CascadeClassifier(self.as_raw_mut_CParams(), val.as_raw_mut_CascadeClassifier()) }.into_result().expect("Infallible function failed: set_face_cascade")
	}
	
}

pub struct CParams {
	ptr: *mut c_void
}

opencv_type_boxed! { CParams }

impl Drop for CParams {
	fn drop(&mut self) {
		extern "C" { fn cv_CParams_delete(instance: *mut c_void); }
		unsafe { cv_CParams_delete(self.as_raw_mut_CParams()) };
	}
}

impl CParams {
	#[inline] pub fn as_raw_CParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CParams {}

impl crate::face::CParamsTrait for CParams {
	#[inline] fn as_raw_CParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CParams {
	/// ## C++ default parameters
	/// * sf: 1.1
	/// * min_n: 3
	/// * min_sz: Size(30,30)
	/// * max_sz: Size()
	pub fn new(cascade_model: &str, sf: f64, min_n: i32, min_sz: core::Size, max_sz: core::Size) -> Result<crate::face::CParams> {
		extern_container_arg!(mut cascade_model);
		unsafe { sys::cv_face_CParams_CParams_String_double_int_Size_Size(cascade_model.opencv_as_extern_mut(), sf, min_n, min_sz.opencv_as_extern(), max_sz.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::face::CParams::opencv_from_extern(r) } )
	}
	
}

pub trait EigenFaceRecognizer: crate::face::BasicFaceRecognizer {
	fn as_raw_EigenFaceRecognizer(&self) -> *const c_void;
	fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void;

}

impl dyn EigenFaceRecognizer + '_ {
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
	pub fn create(num_components: i32, threshold: f64) -> Result<core::Ptr::<dyn crate::face::EigenFaceRecognizer>> {
		unsafe { sys::cv_face_EigenFaceRecognizer_create_int_double(num_components, threshold) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::EigenFaceRecognizer>::opencv_from_extern(r) } )
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
/// ```ignore
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
/// ```ignore
/// // The following line reads the threshold from the Eigenfaces model:
/// double current_threshold = model->getDouble("threshold");
/// // And this line sets the threshold to 0.0:
/// model->set("threshold", 0.0);
/// ```
/// 
/// 
/// If you've set the threshold to 0.0 as we did above, then:
/// 
/// ```ignore
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
/// ```ignore
/// // Create a FaceRecognizer:
/// Ptr<FaceRecognizer> model = EigenFaceRecognizer::create();
/// // And here's how to get its name:
/// String name = model->name();
/// ```
/// 
pub trait FaceRecognizer: core::AlgorithmTrait {
	fn as_raw_FaceRecognizer(&self) -> *const c_void;
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
	/// ```ignore
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
	/// ```ignore
	/// // Create a new Fisherfaces model and retain all available Fisherfaces,
	/// // this is the most common usage of this specific FaceRecognizer:
	/// //
	/// Ptr<FaceRecognizer> model =  FisherFaceRecognizer::create();
	/// ```
	/// 
	/// 
	/// And finally train it on the given dataset (the face images and labels):
	/// 
	/// ```ignore
	/// // This is the common interface to train all of the available cv::FaceRecognizer
	/// // implementations:
	/// //
	/// model->train(images, labels);
	/// ```
	/// 
	fn train(&mut self, src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(labels);
		unsafe { sys::cv_face_FaceRecognizer_train_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray()) }.into_result()
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
	/// ```ignore
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
	/// ```ignore
	/// OpenCV Error: The function/feature is not implemented (This FaceRecognizer (FaceRecognizer.Eigenfaces) does not support updating, you have to use FaceRecognizer::train to update it.) in update, file /home/philipp/git/opencv/modules/contrib/src/facerec.cpp, line 305
	/// terminate called after throwing an instance of "cv::Exception"
	/// ```
	/// 
	/// 
	/// 
	/// Note: The FaceRecognizer does not store your training images, because this would be very
	/// memory intense and it's not the responsibility of te FaceRecognizer to do so. The caller is
	/// responsible for maintaining the dataset, he want to work with.
	fn update(&mut self, src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(labels);
		unsafe { sys::cv_face_FaceRecognizer_update_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray()) }.into_result()
	}
	
	/// - if implemented - send all result of prediction to collector that can be used for somehow custom result handling
	/// ## Parameters
	/// * src: Sample image to get a prediction from.
	/// * collector: User-defined collector object that accepts all results
	/// 
	/// To implement this method u just have to do same internal cycle as in predict(InputArray src, CV_OUT int &label, CV_OUT double &confidence) but
	/// not try to get "best@ result, just resend it to caller side with given collector
	/// 
	/// ## Overloaded parameters
	fn predict_label(&self, src: &dyn core::ToInputArray) -> Result<i32> {
		input_array_arg!(src);
		unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR(self.as_raw_FaceRecognizer(), src.as_raw__InputArray()) }.into_result()
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
	/// ```ignore
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
	/// ```ignore
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
	fn predict(&self, src: &dyn core::ToInputArray, label: &mut i32, confidence: &mut f64) -> Result<()> {
		input_array_arg!(src);
		unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR_intR_doubleR(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), label, confidence) }.into_result()
	}
	
	/// - if implemented - send all result of prediction to collector that can be used for somehow custom result handling
	/// ## Parameters
	/// * src: Sample image to get a prediction from.
	/// * collector: User-defined collector object that accepts all results
	/// 
	/// To implement this method u just have to do same internal cycle as in predict(InputArray src, CV_OUT int &label, CV_OUT double &confidence) but
	/// not try to get "best@ result, just resend it to caller side with given collector
	fn predict_collect(&self, src: &dyn core::ToInputArray, mut collector: core::Ptr::<dyn crate::face::PredictCollector>) -> Result<()> {
		input_array_arg!(src);
		unsafe { sys::cv_face_FaceRecognizer_predict_const_const__InputArrayR_Ptr_PredictCollector_(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), collector.as_raw_mut_PtrOfPredictCollector()) }.into_result()
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
	fn write(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		unsafe { sys::cv_face_FaceRecognizer_write_const_const_StringR(self.as_raw_FaceRecognizer(), filename.opencv_as_extern()) }.into_result()
	}
	
	/// Loads a FaceRecognizer and its model state.
	/// 
	/// Loads a persisted model and state from a given XML or YAML file . Every FaceRecognizer has to
	/// overwrite FaceRecognizer::load(FileStorage& fs) to enable loading the model state.
	/// FaceRecognizer::load(FileStorage& fs) in turn gets called by
	/// FaceRecognizer::load(const String& filename), to ease saving a model.
	fn read(&mut self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		unsafe { sys::cv_face_FaceRecognizer_read_const_StringR(self.as_raw_mut_FaceRecognizer(), filename.opencv_as_extern()) }.into_result()
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
	fn write_1(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_face_FaceRecognizer_write_const_FileStorageR(self.as_raw_FaceRecognizer(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// Loads a FaceRecognizer and its model state.
	/// 
	/// Loads a persisted model and state from a given XML or YAML file . Every FaceRecognizer has to
	/// overwrite FaceRecognizer::load(FileStorage& fs) to enable loading the model state.
	/// FaceRecognizer::load(FileStorage& fs) in turn gets called by
	/// FaceRecognizer::load(const String& filename), to ease saving a model.
	/// 
	/// ## Overloaded parameters
	fn read_1(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_face_FaceRecognizer_read_const_FileNodeR(self.as_raw_mut_FaceRecognizer(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_face_FaceRecognizer_empty_const(self.as_raw_FaceRecognizer()) }.into_result()
	}
	
	/// Sets string info for the specified model's label.
	/// 
	/// The string info is replaced by the provided value if it was set before for the specified label.
	fn set_label_info(&mut self, label: i32, str_info: &str) -> Result<()> {
		extern_container_arg!(str_info);
		unsafe { sys::cv_face_FaceRecognizer_setLabelInfo_int_const_StringR(self.as_raw_mut_FaceRecognizer(), label, str_info.opencv_as_extern()) }.into_result()
	}
	
	/// Gets string information by label.
	/// 
	/// If an unknown label id is provided or there is no label information associated with the specified
	/// label id the method returns an empty string.
	fn get_label_info(&self, label: i32) -> Result<String> {
		unsafe { sys::cv_face_FaceRecognizer_getLabelInfo_const_int(self.as_raw_FaceRecognizer(), label) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Gets vector of labels by string.
	/// 
	/// The function searches for the labels containing the specified sub-string in the associated string
	/// info.
	fn get_labels_by_string(&self, str: &str) -> Result<core::Vector::<i32>> {
		extern_container_arg!(str);
		unsafe { sys::cv_face_FaceRecognizer_getLabelsByString_const_const_StringR(self.as_raw_FaceRecognizer(), str.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
	}
	
	/// threshold parameter accessor - required for default BestMinDist collector
	fn get_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_face_FaceRecognizer_getThreshold_const(self.as_raw_FaceRecognizer()) }.into_result()
	}
	
	/// Sets threshold of model
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_face_FaceRecognizer_setThreshold_double(self.as_raw_mut_FaceRecognizer(), val) }.into_result()
	}
	
}

/// Abstract base class for all facemark models
/// 
/// To utilize this API in your program, please take a look at the @ref tutorial_table_of_content_facemark
/// ### Description
/// 
/// Facemark is a base class which provides universal access to any specific facemark algorithm.
/// Therefore, the users should declare a desired algorithm before they can use it in their application.
/// 
/// Here is an example on how to declare a facemark algorithm:
/// ```ignore
/// // Using Facemark in your code:
/// Ptr<Facemark> facemark = createFacemarkLBF();
/// ```
/// 
/// 
/// The typical pipeline for facemark detection is as follows:
/// - Load the trained model using Facemark::loadModel.
/// - Perform the fitting on an image via Facemark::fit.
pub trait Facemark: core::AlgorithmTrait {
	fn as_raw_Facemark(&self) -> *const c_void;
	fn as_raw_mut_Facemark(&mut self) -> *mut c_void;

	/// A function to load the trained model before the fitting process.
	/// ## Parameters
	/// * model: A string represent the filename of a trained model.
	/// 
	/// <B>Example of usage</B>
	/// ```ignore
	/// facemark->loadModel("../data/lbf.model");
	/// ```
	/// 
	fn load_model(&mut self, model: &str) -> Result<()> {
		extern_container_arg!(mut model);
		unsafe { sys::cv_face_Facemark_loadModel_String(self.as_raw_mut_Facemark(), model.opencv_as_extern_mut()) }.into_result()
	}
	
	/// Detect facial landmarks from an image.
	/// ## Parameters
	/// * image: Input image.
	/// * faces: Output of the function which represent region of interest of the detected faces.
	/// Each face is stored in cv::Rect container.
	/// * landmarks: The detected landmark points for each faces.
	/// 
	/// <B>Example of usage</B>
	/// ```ignore
	/// Mat image = imread("image.jpg");
	/// std::vector<Rect> faces;
	/// std::vector<std::vector<Point2f> > landmarks;
	/// facemark->fit(image, faces, landmarks);
	/// ```
	/// 
	fn fit(&mut self, image: &dyn core::ToInputArray, faces: &dyn core::ToInputArray, landmarks: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(faces);
		output_array_arg!(landmarks);
		unsafe { sys::cv_face_Facemark_fit_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Facemark(), image.as_raw__InputArray(), faces.as_raw__InputArray(), landmarks.as_raw__OutputArray()) }.into_result()
	}
	
}

pub trait FacemarkAAM: crate::face::FacemarkTrain {
	fn as_raw_FacemarkAAM(&self) -> *const c_void;
	fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void;

	/// overload with additional Config structures
	fn fit_config(&mut self, image: &dyn core::ToInputArray, roi: &dyn core::ToInputArray, _landmarks: &mut dyn core::ToOutputArray, runtime_params: &core::Vector::<crate::face::FacemarkAAM_Config>) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(roi);
		output_array_arg!(_landmarks);
		unsafe { sys::cv_face_FacemarkAAM_fitConfig_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_vector_Config_R(self.as_raw_mut_FacemarkAAM(), image.as_raw__InputArray(), roi.as_raw__InputArray(), _landmarks.as_raw__OutputArray(), runtime_params.as_raw_VectorOfFacemarkAAM_Config()) }.into_result()
	}
	
}

impl dyn FacemarkAAM + '_ {
	/// initializer
	/// 
	/// ## C++ default parameters
	/// * parameters: FacemarkAAM::Params()
	pub fn create(parameters: &crate::face::FacemarkAAM_Params) -> Result<core::Ptr::<dyn crate::face::FacemarkAAM>> {
		unsafe { sys::cv_face_FacemarkAAM_create_const_ParamsR(parameters.as_raw_FacemarkAAM_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::FacemarkAAM>::opencv_from_extern(r) } )
	}
	
}
/// \brief Optional parameter for fitting process.
pub trait FacemarkAAM_ConfigTrait {
	fn as_raw_FacemarkAAM_Config(&self) -> *const c_void;
	fn as_raw_mut_FacemarkAAM_Config(&mut self) -> *mut c_void;

	fn r(&mut self) -> core::Mat {
		unsafe { sys::cv_face_FacemarkAAM_Config_getPropR(self.as_raw_mut_FacemarkAAM_Config()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: r")
	}
	
	fn set_r(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Config_setPropR_Mat(self.as_raw_mut_FacemarkAAM_Config(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_r")
	}
	
	fn t(&self) -> core::Point2f {
		unsafe { sys::cv_face_FacemarkAAM_Config_getPropT_const(self.as_raw_FacemarkAAM_Config()) }.into_result().expect("Infallible function failed: t")
	}
	
	fn set_t(&mut self, val: core::Point2f) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Config_setPropT_Point2f(self.as_raw_mut_FacemarkAAM_Config(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_t")
	}
	
	fn scale(&self) -> f32 {
		unsafe { sys::cv_face_FacemarkAAM_Config_getPropScale_const(self.as_raw_FacemarkAAM_Config()) }.into_result().expect("Infallible function failed: scale")
	}
	
	fn set_scale(&mut self, val: f32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Config_setPropScale_float(self.as_raw_mut_FacemarkAAM_Config(), val) }.into_result().expect("Infallible function failed: set_scale")
	}
	
	fn model_scale_idx(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Config_getPropModel_scale_idx_const(self.as_raw_FacemarkAAM_Config()) }.into_result().expect("Infallible function failed: model_scale_idx")
	}
	
	fn set_model_scale_idx(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Config_setPropModel_scale_idx_int(self.as_raw_mut_FacemarkAAM_Config(), val) }.into_result().expect("Infallible function failed: set_model_scale_idx")
	}
	
}

/// \brief Optional parameter for fitting process.
pub struct FacemarkAAM_Config {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Config }

impl Drop for FacemarkAAM_Config {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Config_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Config_delete(self.as_raw_mut_FacemarkAAM_Config()) };
	}
}

impl FacemarkAAM_Config {
	#[inline] pub fn as_raw_FacemarkAAM_Config(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FacemarkAAM_Config(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FacemarkAAM_Config {}

impl crate::face::FacemarkAAM_ConfigTrait for FacemarkAAM_Config {
	#[inline] fn as_raw_FacemarkAAM_Config(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FacemarkAAM_Config(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Config {
	/// ## C++ default parameters
	/// * rot: Mat::eye(2,2,CV_32F)
	/// * trans: Point2f(0.0f,0.0f)
	/// * scaling: 1.0f
	/// * scale_id: 0
	pub fn new(mut rot: core::Mat, trans: core::Point2f, scaling: f32, scale_id: i32) -> Result<crate::face::FacemarkAAM_Config> {
		unsafe { sys::cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(rot.as_raw_mut_Mat(), trans.opencv_as_extern(), scaling, scale_id) }.into_result().map(|r| unsafe { crate::face::FacemarkAAM_Config::opencv_from_extern(r) } )
	}
	
}

/// \brief Data container for the facemark::getData function
pub trait FacemarkAAM_DataTrait {
	fn as_raw_FacemarkAAM_Data(&self) -> *const c_void;
	fn as_raw_mut_FacemarkAAM_Data(&mut self) -> *mut c_void;

	fn s0(&mut self) -> core::Vector::<core::Point2f> {
		unsafe { sys::cv_face_FacemarkAAM_Data_getPropS0(self.as_raw_mut_FacemarkAAM_Data()) }.into_result().map(|r| unsafe { core::Vector::<core::Point2f>::opencv_from_extern(r) } ).expect("Infallible function failed: s0")
	}
	
	fn set_s0(&mut self, mut val: core::Vector::<core::Point2f>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Data_setPropS0_vector_Point2f_(self.as_raw_mut_FacemarkAAM_Data(), val.as_raw_mut_VectorOfPoint2f()) }.into_result().expect("Infallible function failed: set_s0")
	}
	
}

/// \brief Data container for the facemark::getData function
pub struct FacemarkAAM_Data {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Data }

impl Drop for FacemarkAAM_Data {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Data_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Data_delete(self.as_raw_mut_FacemarkAAM_Data()) };
	}
}

impl FacemarkAAM_Data {
	#[inline] pub fn as_raw_FacemarkAAM_Data(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FacemarkAAM_Data(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FacemarkAAM_Data {}

impl crate::face::FacemarkAAM_DataTrait for FacemarkAAM_Data {
	#[inline] fn as_raw_FacemarkAAM_Data(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FacemarkAAM_Data(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Data {
}

/// \brief The model of AAM Algorithm
pub trait FacemarkAAM_ModelTrait {
	fn as_raw_FacemarkAAM_Model(&self) -> *const c_void;
	fn as_raw_mut_FacemarkAAM_Model(&mut self) -> *mut c_void;

	fn scales(&mut self) -> core::Vector::<f32> {
		unsafe { sys::cv_face_FacemarkAAM_Model_getPropScales(self.as_raw_mut_FacemarkAAM_Model()) }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } ).expect("Infallible function failed: scales")
	}
	
	fn set_scales(&mut self, mut val: core::Vector::<f32>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_setPropScales_vector_float_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOff32()) }.into_result().expect("Infallible function failed: set_scales")
	}
	
	fn triangles(&mut self) -> core::Vector::<core::Vec3i> {
		unsafe { sys::cv_face_FacemarkAAM_Model_getPropTriangles(self.as_raw_mut_FacemarkAAM_Model()) }.into_result().map(|r| unsafe { core::Vector::<core::Vec3i>::opencv_from_extern(r) } ).expect("Infallible function failed: triangles")
	}
	
	fn set_triangles(&mut self, mut val: core::Vector::<core::Vec3i>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_setPropTriangles_vector_Vec3i_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfVec3i()) }.into_result().expect("Infallible function failed: set_triangles")
	}
	
	fn textures(&mut self) -> core::Vector::<crate::face::FacemarkAAM_Model_Texture> {
		unsafe { sys::cv_face_FacemarkAAM_Model_getPropTextures(self.as_raw_mut_FacemarkAAM_Model()) }.into_result().map(|r| unsafe { core::Vector::<crate::face::FacemarkAAM_Model_Texture>::opencv_from_extern(r) } ).expect("Infallible function failed: textures")
	}
	
	fn set_textures(&mut self, mut val: core::Vector::<crate::face::FacemarkAAM_Model_Texture>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_setPropTextures_vector_Texture_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfFacemarkAAM_Model_Texture()) }.into_result().expect("Infallible function failed: set_textures")
	}
	
	fn s0(&mut self) -> core::Vector::<core::Point2f> {
		unsafe { sys::cv_face_FacemarkAAM_Model_getPropS0(self.as_raw_mut_FacemarkAAM_Model()) }.into_result().map(|r| unsafe { core::Vector::<core::Point2f>::opencv_from_extern(r) } ).expect("Infallible function failed: s0")
	}
	
	fn set_s0(&mut self, mut val: core::Vector::<core::Point2f>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_setPropS0_vector_Point2f_(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_VectorOfPoint2f()) }.into_result().expect("Infallible function failed: set_s0")
	}
	
	fn s(&mut self) -> core::Mat {
		unsafe { sys::cv_face_FacemarkAAM_Model_getPropS(self.as_raw_mut_FacemarkAAM_Model()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: s")
	}
	
	fn set_s(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_setPropS_Mat(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_s")
	}
	
	fn q(&mut self) -> core::Mat {
		unsafe { sys::cv_face_FacemarkAAM_Model_getPropQ(self.as_raw_mut_FacemarkAAM_Model()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: q")
	}
	
	fn set_q(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_setPropQ_Mat(self.as_raw_mut_FacemarkAAM_Model(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_q")
	}
	
}

/// \brief The model of AAM Algorithm
pub struct FacemarkAAM_Model {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Model }

impl Drop for FacemarkAAM_Model {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Model_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Model_delete(self.as_raw_mut_FacemarkAAM_Model()) };
	}
}

impl FacemarkAAM_Model {
	#[inline] pub fn as_raw_FacemarkAAM_Model(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FacemarkAAM_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FacemarkAAM_Model {}

impl crate::face::FacemarkAAM_ModelTrait for FacemarkAAM_Model {
	#[inline] fn as_raw_FacemarkAAM_Model(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FacemarkAAM_Model(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Model {
}

pub trait FacemarkAAM_Model_TextureTrait {
	fn as_raw_FacemarkAAM_Model_Texture(&self) -> *const c_void;
	fn as_raw_mut_FacemarkAAM_Model_Texture(&mut self) -> *mut c_void;

	/// unused delete
	fn max_m(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropMax_m_const(self.as_raw_FacemarkAAM_Model_Texture()) }.into_result().expect("Infallible function failed: max_m")
	}
	
	/// unused delete
	fn set_max_m(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropMax_m_int(self.as_raw_mut_FacemarkAAM_Model_Texture(), val) }.into_result().expect("Infallible function failed: set_max_m")
	}
	
	fn resolution(&self) -> core::Rect {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropResolution_const(self.as_raw_FacemarkAAM_Model_Texture()) }.into_result().expect("Infallible function failed: resolution")
	}
	
	fn set_resolution(&mut self, val: core::Rect) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropResolution_Rect(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_resolution")
	}
	
	fn a(&mut self) -> core::Mat {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropA(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: a")
	}
	
	fn set_a(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropA_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_a")
	}
	
	fn a0(&mut self) -> core::Mat {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropA0(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: a0")
	}
	
	fn set_a0(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropA0_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_a0")
	}
	
	fn aa(&mut self) -> core::Mat {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropAA(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: aa")
	}
	
	fn set_aa(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropAA_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_aa")
	}
	
	fn aa0(&mut self) -> core::Mat {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropAA0(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: aa0")
	}
	
	fn set_aa0(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropAA0_Mat(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_aa0")
	}
	
	fn texture_idx(&mut self) -> core::Vector::<core::Vector::<core::Point>> {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropTextureIdx(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Vector::<core::Vector::<core::Point>>::opencv_from_extern(r) } ).expect("Infallible function failed: texture_idx")
	}
	
	fn set_texture_idx(&mut self, mut val: core::Vector::<core::Vector::<core::Point>>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropTextureIdx_vector_vector_Point__(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfVectorOfPoint()) }.into_result().expect("Infallible function failed: set_texture_idx")
	}
	
	fn base_shape(&mut self) -> core::Vector::<core::Point2f> {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropBase_shape(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Vector::<core::Point2f>::opencv_from_extern(r) } ).expect("Infallible function failed: base_shape")
	}
	
	fn set_base_shape(&mut self, mut val: core::Vector::<core::Point2f>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropBase_shape_vector_Point2f_(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfPoint2f()) }.into_result().expect("Infallible function failed: set_base_shape")
	}
	
	fn ind1(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropInd1(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: ind1")
	}
	
	fn set_ind1(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropInd1_vector_int_(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_ind1")
	}
	
	fn ind2(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_getPropInd2(self.as_raw_mut_FacemarkAAM_Model_Texture()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: ind2")
	}
	
	fn set_ind2(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Model_Texture_setPropInd2_vector_int_(self.as_raw_mut_FacemarkAAM_Model_Texture(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_ind2")
	}
	
}

pub struct FacemarkAAM_Model_Texture {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Model_Texture }

impl Drop for FacemarkAAM_Model_Texture {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Model_Texture_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Model_Texture_delete(self.as_raw_mut_FacemarkAAM_Model_Texture()) };
	}
}

impl FacemarkAAM_Model_Texture {
	#[inline] pub fn as_raw_FacemarkAAM_Model_Texture(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FacemarkAAM_Model_Texture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FacemarkAAM_Model_Texture {}

impl crate::face::FacemarkAAM_Model_TextureTrait for FacemarkAAM_Model_Texture {
	#[inline] fn as_raw_FacemarkAAM_Model_Texture(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FacemarkAAM_Model_Texture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Model_Texture {
}

pub trait FacemarkAAM_ParamsTrait {
	fn as_raw_FacemarkAAM_Params(&self) -> *const c_void;
	fn as_raw_mut_FacemarkAAM_Params(&mut self) -> *mut c_void;

	fn model_filename(&self) -> String {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropModel_filename_const(self.as_raw_FacemarkAAM_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: model_filename")
	}
	
	fn set_model_filename(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropModel_filename_string(self.as_raw_mut_FacemarkAAM_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_model_filename")
	}
	
	fn m(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropM_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: m")
	}
	
	fn set_m(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropM_int(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_m")
	}
	
	fn n(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropN_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: n")
	}
	
	fn set_n(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropN_int(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_n")
	}
	
	fn n_iter(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropN_iter_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: n_iter")
	}
	
	fn set_n_iter(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropN_iter_int(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_n_iter")
	}
	
	fn verbose(&self) -> bool {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropVerbose_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: verbose")
	}
	
	fn set_verbose(&mut self, val: bool) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropVerbose_bool(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_verbose")
	}
	
	fn save_model(&self) -> bool {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropSave_model_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: save_model")
	}
	
	fn set_save_model(&mut self, val: bool) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropSave_model_bool(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_save_model")
	}
	
	fn max_m(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropMax_m_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: max_m")
	}
	
	fn set_max_m(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropMax_m_int(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_max_m")
	}
	
	fn max_n(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropMax_n_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: max_n")
	}
	
	fn set_max_n(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropMax_n_int(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_max_n")
	}
	
	fn texture_max_m(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropTexture_max_m_const(self.as_raw_FacemarkAAM_Params()) }.into_result().expect("Infallible function failed: texture_max_m")
	}
	
	fn set_texture_max_m(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropTexture_max_m_int(self.as_raw_mut_FacemarkAAM_Params(), val) }.into_result().expect("Infallible function failed: set_texture_max_m")
	}
	
	fn scales(&mut self) -> core::Vector::<f32> {
		unsafe { sys::cv_face_FacemarkAAM_Params_getPropScales(self.as_raw_mut_FacemarkAAM_Params()) }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } ).expect("Infallible function failed: scales")
	}
	
	fn set_scales(&mut self, mut val: core::Vector::<f32>) -> () {
		unsafe { sys::cv_face_FacemarkAAM_Params_setPropScales_vector_float_(self.as_raw_mut_FacemarkAAM_Params(), val.as_raw_mut_VectorOff32()) }.into_result().expect("Infallible function failed: set_scales")
	}
	
	/// \brief Read parameters from file, currently unused
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_face_FacemarkAAM_Params_read_const_FileNodeR(self.as_raw_mut_FacemarkAAM_Params(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	/// \brief Read parameters from file, currently unused
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_face_FacemarkAAM_Params_write_const_FileStorageR(self.as_raw_FacemarkAAM_Params(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct FacemarkAAM_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkAAM_Params }

impl Drop for FacemarkAAM_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkAAM_Params_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkAAM_Params_delete(self.as_raw_mut_FacemarkAAM_Params()) };
	}
}

impl FacemarkAAM_Params {
	#[inline] pub fn as_raw_FacemarkAAM_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FacemarkAAM_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FacemarkAAM_Params {}

impl crate::face::FacemarkAAM_ParamsTrait for FacemarkAAM_Params {
	#[inline] fn as_raw_FacemarkAAM_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FacemarkAAM_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkAAM_Params {
	/// \brief Constructor
	pub fn default() -> Result<crate::face::FacemarkAAM_Params> {
		unsafe { sys::cv_face_FacemarkAAM_Params_Params() }.into_result().map(|r| unsafe { crate::face::FacemarkAAM_Params::opencv_from_extern(r) } )
	}
	
}

pub trait FacemarkKazemi: crate::face::Facemark {
	fn as_raw_FacemarkKazemi(&self) -> *const c_void;
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
	fn training(&mut self, images: &mut core::Vector::<core::Mat>, landmarks: &mut core::Vector::<core::Vector::<core::Point2f>>, configfile: &str, scale: core::Size, model_filename: &str) -> Result<bool> {
		extern_container_arg!(mut configfile);
		extern_container_arg!(mut model_filename);
		unsafe { sys::cv_face_FacemarkKazemi_training_vector_Mat_R_vector_vector_Point2f__R_string_Size_string(self.as_raw_mut_FacemarkKazemi(), images.as_raw_mut_VectorOfMat(), landmarks.as_raw_mut_VectorOfVectorOfPoint2f(), configfile.opencv_as_extern_mut(), scale.opencv_as_extern(), model_filename.opencv_as_extern_mut()) }.into_result()
	}
	
	/// set the custom face detector
	fn set_face_detector(&mut self, f: Option<Box<dyn FnMut(*const c_void, *const c_void) -> bool + Send + Sync + 'static>>) -> Result<bool> {
		callback_arg!(f_trampoline(unnamed: *const c_void, unnamed_1: *const c_void, unnamed_2: *mut c_void) -> bool => unnamed_2 in callbacks => f(unnamed: *const c_void, unnamed_1: *const c_void) -> bool);
		userdata_arg!(user_data in callbacks => f);
		unsafe { sys::cv_face_FacemarkKazemi_setFaceDetector_bool__X__const_cv__InputArrayR__const_cv__OutputArrayR__voidX__voidX(self.as_raw_mut_FacemarkKazemi(), f_trampoline, user_data) }.into_result()
	}
	
	/// get faces using the custom detector
	fn get_faces(&mut self, image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(faces);
		unsafe { sys::cv_face_FacemarkKazemi_getFaces_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FacemarkKazemi(), image.as_raw__InputArray(), faces.as_raw__OutputArray()) }.into_result()
	}
	
}

impl dyn FacemarkKazemi + '_ {
	/// ## C++ default parameters
	/// * parameters: FacemarkKazemi::Params()
	pub fn create(parameters: &crate::face::FacemarkKazemi_Params) -> Result<core::Ptr::<dyn crate::face::FacemarkKazemi>> {
		unsafe { sys::cv_face_FacemarkKazemi_create_const_ParamsR(parameters.as_raw_FacemarkKazemi_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::FacemarkKazemi>::opencv_from_extern(r) } )
	}
	
}
pub trait FacemarkKazemi_ParamsTrait {
	fn as_raw_FacemarkKazemi_Params(&self) -> *const c_void;
	fn as_raw_mut_FacemarkKazemi_Params(&mut self) -> *mut c_void;

	/// cascade_depth This stores the deapth of cascade used for training.
	fn cascade_depth(&self) -> u32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropCascade_depth_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: cascade_depth")
	}
	
	/// cascade_depth This stores the deapth of cascade used for training.
	fn set_cascade_depth(&mut self, val: u32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropCascade_depth_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_cascade_depth")
	}
	
	/// tree_depth This stores the max height of the regression tree built.
	fn tree_depth(&self) -> u32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropTree_depth_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: tree_depth")
	}
	
	/// tree_depth This stores the max height of the regression tree built.
	fn set_tree_depth(&mut self, val: u32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropTree_depth_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_tree_depth")
	}
	
	/// num_trees_per_cascade_level This stores number of trees fit per cascade level.
	fn num_trees_per_cascade_level(&self) -> u32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropNum_trees_per_cascade_level_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: num_trees_per_cascade_level")
	}
	
	/// num_trees_per_cascade_level This stores number of trees fit per cascade level.
	fn set_num_trees_per_cascade_level(&mut self, val: u32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropNum_trees_per_cascade_level_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_num_trees_per_cascade_level")
	}
	
	/// learning_rate stores the learning rate in gradient boosting, also referred as shrinkage.
	fn learning_rate(&self) -> f32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropLearning_rate_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: learning_rate")
	}
	
	/// learning_rate stores the learning rate in gradient boosting, also referred as shrinkage.
	fn set_learning_rate(&mut self, val: f32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropLearning_rate_float(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_learning_rate")
	}
	
	/// oversampling_amount stores number of initialisations used to create training samples.
	fn oversampling_amount(&self) -> u32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropOversampling_amount_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: oversampling_amount")
	}
	
	/// oversampling_amount stores number of initialisations used to create training samples.
	fn set_oversampling_amount(&mut self, val: u32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropOversampling_amount_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_oversampling_amount")
	}
	
	/// num_test_coordinates stores number of test coordinates.
	fn num_test_coordinates(&self) -> u32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropNum_test_coordinates_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: num_test_coordinates")
	}
	
	/// num_test_coordinates stores number of test coordinates.
	fn set_num_test_coordinates(&mut self, val: u32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropNum_test_coordinates_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_num_test_coordinates")
	}
	
	/// lambda stores a value to calculate probability of closeness of two coordinates.
	fn lambda(&self) -> f32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropLambda_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: lambda")
	}
	
	/// lambda stores a value to calculate probability of closeness of two coordinates.
	fn set_lambda(&mut self, val: f32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropLambda_float(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_lambda")
	}
	
	/// num_test_splits stores number of random test splits generated.
	fn num_test_splits(&self) -> u32 {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropNum_test_splits_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().expect("Infallible function failed: num_test_splits")
	}
	
	/// num_test_splits stores number of random test splits generated.
	fn set_num_test_splits(&mut self, val: u32) -> () {
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropNum_test_splits_unsigned_long(self.as_raw_mut_FacemarkKazemi_Params(), val) }.into_result().expect("Infallible function failed: set_num_test_splits")
	}
	
	/// configfile stores the name of the file containing the values of training parameters
	fn configfile(&self) -> String {
		unsafe { sys::cv_face_FacemarkKazemi_Params_getPropConfigfile_const(self.as_raw_FacemarkKazemi_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: configfile")
	}
	
	/// configfile stores the name of the file containing the values of training parameters
	fn set_configfile(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_face_FacemarkKazemi_Params_setPropConfigfile_String(self.as_raw_mut_FacemarkKazemi_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_configfile")
	}
	
}

pub struct FacemarkKazemi_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkKazemi_Params }

impl Drop for FacemarkKazemi_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkKazemi_Params_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkKazemi_Params_delete(self.as_raw_mut_FacemarkKazemi_Params()) };
	}
}

impl FacemarkKazemi_Params {
	#[inline] pub fn as_raw_FacemarkKazemi_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FacemarkKazemi_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FacemarkKazemi_Params {}

impl crate::face::FacemarkKazemi_ParamsTrait for FacemarkKazemi_Params {
	#[inline] fn as_raw_FacemarkKazemi_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FacemarkKazemi_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkKazemi_Params {
	/// \brief Constructor
	pub fn default() -> Result<crate::face::FacemarkKazemi_Params> {
		unsafe { sys::cv_face_FacemarkKazemi_Params_Params() }.into_result().map(|r| unsafe { crate::face::FacemarkKazemi_Params::opencv_from_extern(r) } )
	}
	
}

pub trait FacemarkLBF: crate::face::FacemarkTrain {
	fn as_raw_FacemarkLBF(&self) -> *const c_void;
	fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void;

}

impl dyn FacemarkLBF + '_ {
	/// ## C++ default parameters
	/// * parameters: FacemarkLBF::Params()
	pub fn create(parameters: &crate::face::FacemarkLBF_Params) -> Result<core::Ptr::<dyn crate::face::FacemarkLBF>> {
		unsafe { sys::cv_face_FacemarkLBF_create_const_ParamsR(parameters.as_raw_FacemarkLBF_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::FacemarkLBF>::opencv_from_extern(r) } )
	}
	
}
pub trait FacemarkLBF_ParamsTrait {
	fn as_raw_FacemarkLBF_Params(&self) -> *const c_void;
	fn as_raw_mut_FacemarkLBF_Params(&mut self) -> *mut c_void;

	fn shape_offset(&self) -> f64 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropShape_offset_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: shape_offset")
	}
	
	fn set_shape_offset(&mut self, val: f64) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropShape_offset_double(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_shape_offset")
	}
	
	fn cascade_face(&self) -> String {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropCascade_face_const(self.as_raw_FacemarkLBF_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: cascade_face")
	}
	
	fn set_cascade_face(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropCascade_face_String(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_cascade_face")
	}
	
	fn verbose(&self) -> bool {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropVerbose_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: verbose")
	}
	
	fn set_verbose(&mut self, val: bool) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropVerbose_bool(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_verbose")
	}
	
	fn n_landmarks(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropN_landmarks_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: n_landmarks")
	}
	
	fn set_n_landmarks(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropN_landmarks_int(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_n_landmarks")
	}
	
	fn init_shape_n(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropInitShape_n_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: init_shape_n")
	}
	
	fn set_init_shape_n(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropInitShape_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_init_shape_n")
	}
	
	fn stages_n(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropStages_n_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: stages_n")
	}
	
	fn set_stages_n(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropStages_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_stages_n")
	}
	
	fn tree_n(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropTree_n_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: tree_n")
	}
	
	fn set_tree_n(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropTree_n_int(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_tree_n")
	}
	
	fn tree_depth(&self) -> i32 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropTree_depth_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: tree_depth")
	}
	
	fn set_tree_depth(&mut self, val: i32) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropTree_depth_int(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_tree_depth")
	}
	
	fn bagging_overlap(&self) -> f64 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropBagging_overlap_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: bagging_overlap")
	}
	
	fn set_bagging_overlap(&mut self, val: f64) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropBagging_overlap_double(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_bagging_overlap")
	}
	
	fn model_filename(&self) -> String {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropModel_filename_const(self.as_raw_FacemarkLBF_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: model_filename")
	}
	
	fn set_model_filename(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropModel_filename_string(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_model_filename")
	}
	
	/// flag to save the trained model or not
	fn save_model(&self) -> bool {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropSave_model_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: save_model")
	}
	
	/// flag to save the trained model or not
	fn set_save_model(&mut self, val: bool) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropSave_model_bool(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_save_model")
	}
	
	/// seed for shuffling the training data
	fn seed(&self) -> u32 {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropSeed_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: seed")
	}
	
	/// seed for shuffling the training data
	fn set_seed(&mut self, val: u32) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropSeed_unsigned_int(self.as_raw_mut_FacemarkLBF_Params(), val) }.into_result().expect("Infallible function failed: set_seed")
	}
	
	fn feats_m(&mut self) -> core::Vector::<i32> {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropFeats_m(self.as_raw_mut_FacemarkLBF_Params()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } ).expect("Infallible function failed: feats_m")
	}
	
	fn set_feats_m(&mut self, mut val: core::Vector::<i32>) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropFeats_m_vector_int_(self.as_raw_mut_FacemarkLBF_Params(), val.as_raw_mut_VectorOfi32()) }.into_result().expect("Infallible function failed: set_feats_m")
	}
	
	fn radius_m(&mut self) -> core::Vector::<f64> {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropRadius_m(self.as_raw_mut_FacemarkLBF_Params()) }.into_result().map(|r| unsafe { core::Vector::<f64>::opencv_from_extern(r) } ).expect("Infallible function failed: radius_m")
	}
	
	fn set_radius_m(&mut self, mut val: core::Vector::<f64>) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropRadius_m_vector_double_(self.as_raw_mut_FacemarkLBF_Params(), val.as_raw_mut_VectorOff64()) }.into_result().expect("Infallible function failed: set_radius_m")
	}
	
	fn detect_roi(&self) -> core::Rect {
		unsafe { sys::cv_face_FacemarkLBF_Params_getPropDetectROI_const(self.as_raw_FacemarkLBF_Params()) }.into_result().expect("Infallible function failed: detect_roi")
	}
	
	fn set_detect_roi(&mut self, val: core::Rect) -> () {
		unsafe { sys::cv_face_FacemarkLBF_Params_setPropDetectROI_Rect(self.as_raw_mut_FacemarkLBF_Params(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_detect_roi")
	}
	
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_face_FacemarkLBF_Params_read_const_FileNodeR(self.as_raw_mut_FacemarkLBF_Params(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_face_FacemarkLBF_Params_write_const_FileStorageR(self.as_raw_FacemarkLBF_Params(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct FacemarkLBF_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { FacemarkLBF_Params }

impl Drop for FacemarkLBF_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_FacemarkLBF_Params_delete(instance: *mut c_void); }
		unsafe { cv_FacemarkLBF_Params_delete(self.as_raw_mut_FacemarkLBF_Params()) };
	}
}

impl FacemarkLBF_Params {
	#[inline] pub fn as_raw_FacemarkLBF_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_FacemarkLBF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for FacemarkLBF_Params {}

impl crate::face::FacemarkLBF_ParamsTrait for FacemarkLBF_Params {
	#[inline] fn as_raw_FacemarkLBF_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_FacemarkLBF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FacemarkLBF_Params {
	/// \brief Constructor
	pub fn default() -> Result<crate::face::FacemarkLBF_Params> {
		unsafe { sys::cv_face_FacemarkLBF_Params_Params() }.into_result().map(|r| unsafe { crate::face::FacemarkLBF_Params::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for trainable facemark models
/// 
/// To utilize this API in your program, please take a look at the @ref tutorial_table_of_content_facemark
/// ### Description
/// 
/// The AAM and LBF facemark models in OpenCV are derived from the abstract base class FacemarkTrain, which
/// provides a unified access to those facemark algorithms in OpenCV.
/// 
/// Here is an example on how to declare facemark algorithm:
/// ```ignore
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
pub trait FacemarkTrain: crate::face::Facemark {
	fn as_raw_FacemarkTrain(&self) -> *const c_void;
	fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void;

	/// Add one training sample to the trainer.
	/// 
	/// ## Parameters
	/// * image: Input image.
	/// * landmarks: The ground-truth of facial landmarks points corresponds to the image.
	/// 
	/// <B>Example of usage</B>
	/// ```ignore
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
	/// ```ignore
	/// /home/user/ibug/image_003_1.jpg
	/// /home/user/ibug/image_004_1.jpg
	/// /home/user/ibug/image_005_1.jpg
	/// /home/user/ibug/image_006.jpg
	/// ```
	/// 
	/// 
	/// example of content in the points_train.txt
	/// ```ignore
	/// /home/user/ibug/image_003_1.pts
	/// /home/user/ibug/image_004_1.pts
	/// /home/user/ibug/image_005_1.pts
	/// /home/user/ibug/image_006.pts
	/// ```
	/// 
	fn add_training_sample(&mut self, image: &dyn core::ToInputArray, landmarks: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(image);
		input_array_arg!(landmarks);
		unsafe { sys::cv_face_FacemarkTrain_addTrainingSample_const__InputArrayR_const__InputArrayR(self.as_raw_mut_FacemarkTrain(), image.as_raw__InputArray(), landmarks.as_raw__InputArray()) }.into_result()
	}
	
	/// Trains a Facemark algorithm using the given dataset.
	/// Before the training process, training samples should be added to the trainer
	/// using face::addTrainingSample function.
	/// 
	/// ## Parameters
	/// * parameters: Optional extra parameters (algorithm dependent).
	/// 
	/// <B>Example of usage</B>
	/// ```ignore
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
	fn training(&mut self, parameters: *mut c_void) -> Result<()> {
		unsafe { sys::cv_face_FacemarkTrain_training_voidX(self.as_raw_mut_FacemarkTrain(), parameters) }.into_result()
	}
	
	/// Set a user defined face detector for the Facemark algorithm.
	/// ## Parameters
	/// * detector: The user defined face detector function
	/// * userData: Detector parameters
	/// 
	/// <B>Example of usage</B>
	/// ```ignore
	/// MyDetectorParameters detectorParameters(...);
	/// facemark->setFaceDetector(myDetector, &detectorParameters);
	/// ```
	/// 
	/// 
	/// Example of a user defined face detector
	/// ```ignore
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
	fn set_face_detector(&mut self, detector: crate::face::FN_FaceDetector) -> Result<bool> {
		callback_arg!(detector_trampoline(unnamed: *const c_void, unnamed_1: *const c_void, user_data: *mut c_void) -> bool => user_data in callbacks => detector(unnamed: *const c_void, unnamed_1: *const c_void) -> bool);
		userdata_arg!(user_data in callbacks => detector);
		unsafe { sys::cv_face_FacemarkTrain_setFaceDetector_FN_FaceDetector_voidX(self.as_raw_mut_FacemarkTrain(), detector_trampoline, user_data) }.into_result()
	}
	
	/// Detect faces from a given image using default or user defined face detector.
	/// Some Algorithm might not provide a default face detector.
	/// 
	/// ## Parameters
	/// * image: Input image.
	/// * faces: Output of the function which represent region of interest of the detected faces. Each face is stored in cv::Rect container.
	/// 
	/// <B>Example of usage</B>
	/// ```ignore
	/// std::vector<cv::Rect> faces;
	/// facemark->getFaces(img, faces);
	/// for(int j=0;j<faces.size();j++){
	///    cv::rectangle(img, faces[j], cv::Scalar(255,0,255));
	/// }
	/// ```
	/// 
	fn get_faces(&mut self, image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(faces);
		unsafe { sys::cv_face_FacemarkTrain_getFaces_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_FacemarkTrain(), image.as_raw__InputArray(), faces.as_raw__OutputArray()) }.into_result()
	}
	
	/// Get data from an algorithm
	/// 
	/// ## Parameters
	/// * items: The obtained data, algorithm dependent.
	/// 
	/// <B>Example of usage</B>
	/// ```ignore
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
	fn get_data(&mut self, items: *mut c_void) -> Result<bool> {
		unsafe { sys::cv_face_FacemarkTrain_getData_voidX(self.as_raw_mut_FacemarkTrain(), items) }.into_result()
	}
	
}

pub trait FisherFaceRecognizer: crate::face::BasicFaceRecognizer {
	fn as_raw_FisherFaceRecognizer(&self) -> *const c_void;
	fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void;

}

impl dyn FisherFaceRecognizer + '_ {
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
	pub fn create(num_components: i32, threshold: f64) -> Result<core::Ptr::<dyn crate::face::FisherFaceRecognizer>> {
		unsafe { sys::cv_face_FisherFaceRecognizer_create_int_double(num_components, threshold) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::FisherFaceRecognizer>::opencv_from_extern(r) } )
	}
	
}
pub trait LBPHFaceRecognizer: crate::face::FaceRecognizer {
	fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void;
	fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void;

	/// ## See also
	/// setGridX
	fn get_grid_x(&self) -> Result<i32> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_getGridX_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
	}
	
	/// ## See also
	/// setGridX getGridX
	fn set_grid_x(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_setGridX_int(self.as_raw_mut_LBPHFaceRecognizer(), val) }.into_result()
	}
	
	/// ## See also
	/// setGridY
	fn get_grid_y(&self) -> Result<i32> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_getGridY_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
	}
	
	/// ## See also
	/// setGridY getGridY
	fn set_grid_y(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_setGridY_int(self.as_raw_mut_LBPHFaceRecognizer(), val) }.into_result()
	}
	
	/// ## See also
	/// setRadius
	fn get_radius(&self) -> Result<i32> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_getRadius_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
	}
	
	/// ## See also
	/// setRadius getRadius
	fn set_radius(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_setRadius_int(self.as_raw_mut_LBPHFaceRecognizer(), val) }.into_result()
	}
	
	/// ## See also
	/// setNeighbors
	fn get_neighbors(&self) -> Result<i32> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_getNeighbors_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
	}
	
	/// ## See also
	/// setNeighbors getNeighbors
	fn set_neighbors(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_setNeighbors_int(self.as_raw_mut_LBPHFaceRecognizer(), val) }.into_result()
	}
	
	/// ## See also
	/// setThreshold
	fn get_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_getThreshold_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
	}
	
	/// ## See also
	/// setThreshold getThreshold
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_setThreshold_double(self.as_raw_mut_LBPHFaceRecognizer(), val) }.into_result()
	}
	
	fn get_histograms(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_getHistograms_const(self.as_raw_LBPHFaceRecognizer()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	fn get_labels(&self) -> Result<core::Mat> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_getLabels_const(self.as_raw_LBPHFaceRecognizer()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

impl dyn LBPHFaceRecognizer + '_ {
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
	pub fn create(radius: i32, neighbors: i32, grid_x: i32, grid_y: i32, threshold: f64) -> Result<core::Ptr::<dyn crate::face::LBPHFaceRecognizer>> {
		unsafe { sys::cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(radius, neighbors, grid_x, grid_y, threshold) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::LBPHFaceRecognizer>::opencv_from_extern(r) } )
	}
	
}
/// Minimum Average Correlation Energy Filter
///    useful for authentication with (cancellable) biometrical features.
///    (does not need many positives to train (10-50), and no negatives at all, also robust to noise/salting)
/// 
///    see also: [Savvides04](https://docs.opencv.org/4.5.2/d0/de3/citelist.html#CITEREF_Savvides04)
/// 
///    this implementation is largely based on: https://code.google.com/archive/p/pam-face-authentication (GSOC 2009)
/// 
///    use it like:
///    ```ignore
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
///    ```ignore
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
///    ```ignore
///    Ptr<face::MACE> mace = face::MACE::create(64);
///    mace->train(pos_images);
///    mace->save("my_mace.xml");
/// 
///    // later:
///    Ptr<MACE> reloaded = MACE::load("my_mace.xml");
///    reloaded->same(some_image);
///    ```
/// 
pub trait MACE: core::AlgorithmTrait {
	fn as_raw_MACE(&self) -> *const c_void;
	fn as_raw_mut_MACE(&mut self) -> *mut c_void;

	/// optionally encrypt images with random convolution
	/// ## Parameters
	/// * passphrase: a crc64 random seed will get generated from this
	fn salt(&mut self, passphrase: &str) -> Result<()> {
		extern_container_arg!(passphrase);
		unsafe { sys::cv_face_MACE_salt_const_StringR(self.as_raw_mut_MACE(), passphrase.opencv_as_extern()) }.into_result()
	}
	
	/// train it on positive features
	///    compute the mace filter: `h = D(-1) * X * (X(+) * D(-1) * X)(-1) * C`
	///    also calculate a minimal threshold for this class, the smallest self-similarity from the train images
	/// ## Parameters
	/// * images: a vector<Mat> with the train images
	fn train(&mut self, images: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(images);
		unsafe { sys::cv_face_MACE_train_const__InputArrayR(self.as_raw_mut_MACE(), images.as_raw__InputArray()) }.into_result()
	}
	
	/// correlate query img and threshold to min class value
	/// ## Parameters
	/// * query: a Mat with query image
	fn same(&self, query: &dyn core::ToInputArray) -> Result<bool> {
		input_array_arg!(query);
		unsafe { sys::cv_face_MACE_same_const_const__InputArrayR(self.as_raw_MACE(), query.as_raw__InputArray()) }.into_result()
	}
	
}

impl dyn MACE + '_ {
	/// constructor
	/// ## Parameters
	/// * filename: build a new MACE instance from a pre-serialized FileStorage
	/// * objname: (optional) top-level node in the FileStorage
	/// 
	/// ## C++ default parameters
	/// * objname: String()
	pub fn load(filename: &str, objname: &str) -> Result<core::Ptr::<dyn crate::face::MACE>> {
		extern_container_arg!(filename);
		extern_container_arg!(objname);
		unsafe { sys::cv_face_MACE_load_const_StringR_const_StringR(filename.opencv_as_extern(), objname.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::MACE>::opencv_from_extern(r) } )
	}
	
	/// constructor
	/// ## Parameters
	/// * IMGSIZE: images will get resized to this (should be an even number)
	/// 
	/// ## C++ default parameters
	/// * imgsize: 64
	pub fn create(imgsize: i32) -> Result<core::Ptr::<dyn crate::face::MACE>> {
		unsafe { sys::cv_face_MACE_create_int(imgsize) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::face::MACE>::opencv_from_extern(r) } )
	}
	
}
/// Abstract base class for all strategies of prediction result handling
pub trait PredictCollector {
	fn as_raw_PredictCollector(&self) -> *const c_void;
	fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void;

	/// Interface method called by face recognizer before results processing
	/// ## Parameters
	/// * size: total size of prediction evaluation that recognizer could perform
	fn init(&mut self, size: size_t) -> Result<()> {
		unsafe { sys::cv_face_PredictCollector_init_size_t(self.as_raw_mut_PredictCollector(), size) }.into_result()
	}
	
	/// Interface method called by face recognizer for each result
	/// ## Parameters
	/// * label: current prediction label
	/// * dist: current prediction distance (confidence)
	fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
		unsafe { sys::cv_face_PredictCollector_collect_int_double(self.as_raw_mut_PredictCollector(), label, dist) }.into_result()
	}
	
}

/// Default predict collector
/// 
/// Trace minimal distance with treshhold checking (that is default behavior for most predict logic)
pub trait StandardCollectorTrait: crate::face::PredictCollector {
	fn as_raw_StandardCollector(&self) -> *const c_void;
	fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void;

	/// overloaded interface method
	fn init(&mut self, size: size_t) -> Result<()> {
		unsafe { sys::cv_face_StandardCollector_init_size_t(self.as_raw_mut_StandardCollector(), size) }.into_result()
	}
	
	/// overloaded interface method
	fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
		unsafe { sys::cv_face_StandardCollector_collect_int_double(self.as_raw_mut_StandardCollector(), label, dist) }.into_result()
	}
	
	/// Returns label with minimal distance
	fn get_min_label(&self) -> Result<i32> {
		unsafe { sys::cv_face_StandardCollector_getMinLabel_const(self.as_raw_StandardCollector()) }.into_result()
	}
	
	/// Returns minimal distance value
	fn get_min_dist(&self) -> Result<f64> {
		unsafe { sys::cv_face_StandardCollector_getMinDist_const(self.as_raw_StandardCollector()) }.into_result()
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
	fn drop(&mut self) {
		extern "C" { fn cv_StandardCollector_delete(instance: *mut c_void); }
		unsafe { cv_StandardCollector_delete(self.as_raw_mut_StandardCollector()) };
	}
}

impl StandardCollector {
	#[inline] pub fn as_raw_StandardCollector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for StandardCollector {}

impl crate::face::PredictCollector for StandardCollector {
	#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::StandardCollectorTrait for StandardCollector {
	#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StandardCollector {
	/// Constructor
	/// ## Parameters
	/// * threshold_: set threshold
	/// 
	/// ## C++ default parameters
	/// * threshold_: DBL_MAX
	pub fn new(threshold_: f64) -> Result<crate::face::StandardCollector> {
		unsafe { sys::cv_face_StandardCollector_StandardCollector_double(threshold_) }.into_result().map(|r| unsafe { crate::face::StandardCollector::opencv_from_extern(r) } )
	}
	
	/// Static constructor
	/// ## Parameters
	/// * threshold: set threshold
	/// 
	/// ## C++ default parameters
	/// * threshold: DBL_MAX
	pub fn create(threshold: f64) -> Result<core::Ptr::<crate::face::StandardCollector>> {
		unsafe { sys::cv_face_StandardCollector_create_double(threshold) }.into_result().map(|r| unsafe { core::Ptr::<crate::face::StandardCollector>::opencv_from_extern(r) } )
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
	pub fn new(label_: i32, distance_: f64) -> Result<crate::face::StandardCollector_PredictResult> {
		unsafe { sys::cv_face_StandardCollector_PredictResult_PredictResult_int_double(label_, distance_) }.into_result()
	}
	
}

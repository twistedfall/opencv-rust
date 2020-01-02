//! # Face Analysis
//! - @ref tutorial_table_of_content_facemark
//! - The Facemark API
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};


pub type FN_FaceDetector = dyn FnMut(core::_InputArray, core::_OutputArray, &mut c_void) + Send + Sync + 'static;
#[doc(hidden)] pub type FN_FaceDetectorExtern = Option<extern "C" fn(unnamed_arg: *mut c_void, unnamed_arg_1: *mut c_void, user_data: *mut c_void)>;

/// construct an AAM facemark detector
pub fn create_facemark_aam() -> Result<types::PtrOfFacemark> {
    unsafe { sys::cv_face_createFacemarkAAM() }.into_result().map(|ptr| types::PtrOfFacemark { ptr })
}

/// construct a Kazemi facemark detector
pub fn create_facemark_kazemi() -> Result<types::PtrOfFacemark> {
    unsafe { sys::cv_face_createFacemarkKazemi() }.into_result().map(|ptr| types::PtrOfFacemark { ptr })
}

/// construct an LBF facemark detector
pub fn create_facemark_lbf() -> Result<types::PtrOfFacemark> {
    unsafe { sys::cv_face_createFacemarkLBF() }.into_result().map(|ptr| types::PtrOfFacemark { ptr })
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
/// face::drawFacemarks(frame, landmarks[j], Scalar(0,0,255));
/// }
/// ```
///
/// ## C++ default parameters
/// * color: Scalar(255,0,0)
pub fn draw_facemarks(image: &mut dyn core::ToInputOutputArray, points: &dyn core::ToInputArray, color: core::Scalar) -> Result<()> {
    input_output_array_arg!(image);
    input_array_arg!(points);
    unsafe { sys::cv_face_drawFacemarks__InputOutputArray__InputArray_Scalar(image.as_raw__InputOutputArray(), points.as_raw__InputArray(), color) }.into_result()
}

pub fn get_faces_haar(image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray, face_cascade_name: &str) -> Result<bool> {
    input_array_arg!(image);
    output_array_arg!(faces);
    string_arg!(face_cascade_name);
    unsafe { sys::cv_face_getFacesHAAR__InputArray__OutputArray_String(image.as_raw__InputArray(), faces.as_raw__OutputArray(), face_cascade_name.as_ptr()) }.into_result()
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
/// cv::rectangle(frame, faces[j], cv::Scalar(255,0,255));
/// }
/// cv::imshow("detection", frame);
/// ```
pub fn get_faces(image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray, params: &mut crate::face::CParams) -> Result<bool> {
    input_array_arg!(image);
    output_array_arg!(faces);
    unsafe { sys::cv_face_getFaces__InputArray__OutputArray_CParams(image.as_raw__InputArray(), faces.as_raw__OutputArray(), params.as_raw_CParams()) }.into_result()
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
pub fn load_dataset_list(image_list: &str, annotation_list: &str, images: &mut types::VectorOfString, annotations: &mut types::VectorOfString) -> Result<bool> {
    string_arg!(mut image_list);
    string_arg!(mut annotation_list);
    unsafe { sys::cv_face_loadDatasetList_String_String_VectorOfString_VectorOfString(image_list.as_ptr() as _, annotation_list.as_ptr() as _, images.as_raw_VectorOfString(), annotations.as_raw_VectorOfString()) }.into_result()
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
    string_arg!(mut filename);
    output_array_arg!(points);
    unsafe { sys::cv_face_loadFacePoints_String__OutputArray_float(filename.as_ptr() as _, points.as_raw__OutputArray(), offset) }.into_result()
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
/// ## C++ default parameters
/// * offset: 0.0f
pub fn load_training_data(image_list: &str, ground_truth: &str, images: &mut types::VectorOfString, face_points: &mut dyn core::ToOutputArray, offset: f32) -> Result<bool> {
    string_arg!(mut image_list);
    string_arg!(mut ground_truth);
    output_array_arg!(face_points);
    unsafe { sys::cv_face_loadTrainingData_String_String_VectorOfString__OutputArray_float(image_list.as_ptr() as _, ground_truth.as_ptr() as _, images.as_raw_VectorOfString(), face_points.as_raw__OutputArray(), offset) }.into_result()
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
/// ## C++ default parameters
/// * delim: ' '
/// * offset: 0.0f
pub fn load_training_data_1(filename: &str, images: &mut types::VectorOfString, face_points: &mut dyn core::ToOutputArray, delim: i8, offset: f32) -> Result<bool> {
    string_arg!(mut filename);
    output_array_arg!(face_points);
    unsafe { sys::cv_face_loadTrainingData_String_VectorOfString__OutputArray_char_float(filename.as_ptr() as _, images.as_raw_VectorOfString(), face_points.as_raw__OutputArray(), delim, offset) }.into_result()
}

/// This function extracts the data for training from .txt files which contains the corresponding image name and landmarks.
/// *The first file in each file should give the path of the image whose
/// *landmarks are being described in the file. Then in the subsequent
/// *lines there should be coordinates of the landmarks in the image
/// *i.e each line should be of the form x,y
/// *where x represents the x coordinate of the landmark and y represents
/// *the y coordinate of the landmark.
///
/// *For reference you can see the files as provided in the
/// *<a href="http://www.ifp.illinois.edu/~vuongle2/helen/">HELEN dataset</a>
///
/// ## Parameters
/// * filename: A vector of type cv::String containing name of the .txt files.
/// * trainlandmarks: A vector of type cv::Point2f that would store shape or landmarks of all images.
/// * trainimages: A vector of type cv::String which stores the name of images whose landmarks are tracked
/// ## Returns
/// A boolean value. It returns true when it reads the data successfully and false otherwise
pub fn load_training_data_2(filename: &types::VectorOfString, trainlandmarks: &mut types::VectorOfVectorOfPoint2f, trainimages: &mut types::VectorOfString) -> Result<bool> {
    unsafe { sys::cv_face_loadTrainingData_VectorOfString_VectorOfVectorOfPoint2f_VectorOfString(filename.as_raw_VectorOfString(), trainlandmarks.as_raw_VectorOfVectorOfPoint2f(), trainimages.as_raw_VectorOfString()) }.into_result()
}

// Generating impl for trait crate::face::BIF
/// Implementation of bio-inspired features (BIF) from the paper:
///  Guo, Guodong, et al. "Human age estimation using bio-inspired features."
///  Computer Vision and Pattern Recognition, 2009. CVPR 2009.
pub trait BIF: core::AlgorithmTrait {
    fn as_raw_BIF(&self) -> *mut c_void;
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
        unsafe { sys::cv_face_BIF_compute_const__InputArray__OutputArray(self.as_raw_BIF(), image.as_raw__InputArray(), features.as_raw__OutputArray()) }.into_result()
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
    pub fn create(num_bands: i32, num_rotations: i32) -> Result<types::PtrOfBIF> {
        unsafe { sys::cv_face_BIF_create_int_int(num_bands, num_rotations) }.into_result().map(|ptr| types::PtrOfBIF { ptr })
    }
    
}

// Generating impl for trait crate::face::BasicFaceRecognizer
pub trait BasicFaceRecognizerTrait: crate::face::FaceRecognizer {
    fn as_raw_BasicFaceRecognizer(&self) -> *mut c_void;
    /// @see setNumComponents
    fn get_num_components(&self) -> Result<i32> {
        unsafe { sys::cv_face_BasicFaceRecognizer_getNumComponents_const(self.as_raw_BasicFaceRecognizer()) }.into_result()
    }
    
    /// @copybrief getNumComponents @see getNumComponents
    fn set_num_components(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_face_BasicFaceRecognizer_setNumComponents_int(self.as_raw_BasicFaceRecognizer(), val) }.into_result()
    }
    
    /// @see setThreshold
    fn get_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_face_BasicFaceRecognizer_getThreshold_const(self.as_raw_BasicFaceRecognizer()) }.into_result()
    }
    
    /// @copybrief getThreshold @see getThreshold
    fn set_threshold(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_face_BasicFaceRecognizer_setThreshold_double(self.as_raw_BasicFaceRecognizer(), val) }.into_result()
    }
    
    fn get_projections(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_face_BasicFaceRecognizer_getProjections_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn get_labels(&self) -> Result<core::Mat> {
        unsafe { sys::cv_face_BasicFaceRecognizer_getLabels_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_eigen_values(&self) -> Result<core::Mat> {
        unsafe { sys::cv_face_BasicFaceRecognizer_getEigenValues_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_eigen_vectors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_face_BasicFaceRecognizer_getEigenVectors_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_mean(&self) -> Result<core::Mat> {
        unsafe { sys::cv_face_BasicFaceRecognizer_getMean_const(self.as_raw_BasicFaceRecognizer()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn read(&mut self, _fn: &core::FileNode) -> Result<()> {
        unsafe { sys::cv_face_BasicFaceRecognizer_read_FileNode(self.as_raw_BasicFaceRecognizer(), _fn.as_raw_FileNode()) }.into_result()
    }
    
    fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_face_BasicFaceRecognizer_write_const_FileStorage(self.as_raw_BasicFaceRecognizer(), fs.as_raw_FileStorage()) }.into_result()
    }
    
    fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_face_BasicFaceRecognizer_empty_const(self.as_raw_BasicFaceRecognizer()) }.into_result()
    }
    
}

// boxed class cv::face::BasicFaceRecognizer
pub struct BasicFaceRecognizer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BasicFaceRecognizer {
    fn drop(&mut self) {
        unsafe { sys::cv_BasicFaceRecognizer_delete(self.ptr) };
    }
}

impl BasicFaceRecognizer {
    #[inline(always)] pub fn as_raw_BasicFaceRecognizer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BasicFaceRecognizer {}

impl core::AlgorithmTrait for BasicFaceRecognizer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::face::BasicFaceRecognizerTrait for BasicFaceRecognizer {
    #[inline(always)] fn as_raw_BasicFaceRecognizer(&self) -> *mut c_void { self.ptr }
}

impl crate::face::FaceRecognizer for BasicFaceRecognizer {
    #[inline(always)] fn as_raw_FaceRecognizer(&self) -> *mut c_void { self.ptr }
}

// boxed class cv::face::CParams
pub struct CParams {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CParams {
    fn drop(&mut self) {
        unsafe { sys::cv_CParams_delete(self.ptr) };
    }
}

impl CParams {
    #[inline(always)] pub fn as_raw_CParams(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CParams {}

impl CParams {
    ///
    /// ## C++ default parameters
    /// * sf: 1.1
    /// * min_n: 3
    /// * min_sz: Size(30, 30)
    /// * max_sz: Size()
    pub fn new(cascade_model: &str, sf: f64, min_n: i32, min_sz: core::Size, max_sz: core::Size) -> Result<crate::face::CParams> {
        string_arg!(mut cascade_model);
        unsafe { sys::cv_face_CParams_CParams_String_double_int_Size_Size(cascade_model.as_ptr() as _, sf, min_n, min_sz, max_sz) }.into_result().map(|ptr| crate::face::CParams { ptr })
    }
    
}

// boxed class cv::face::EigenFaceRecognizer
pub struct EigenFaceRecognizer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for EigenFaceRecognizer {
    fn drop(&mut self) {
        unsafe { sys::cv_EigenFaceRecognizer_delete(self.ptr) };
    }
}

impl EigenFaceRecognizer {
    #[inline(always)] pub fn as_raw_EigenFaceRecognizer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for EigenFaceRecognizer {}

impl core::AlgorithmTrait for EigenFaceRecognizer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::face::BasicFaceRecognizerTrait for EigenFaceRecognizer {
    #[inline(always)] fn as_raw_BasicFaceRecognizer(&self) -> *mut c_void { self.ptr }
}

impl crate::face::FaceRecognizer for EigenFaceRecognizer {
    #[inline(always)] fn as_raw_FaceRecognizer(&self) -> *mut c_void { self.ptr }
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
    /// color spaces.
    /// *   **THE EIGENFACES METHOD MAKES THE ASSUMPTION, THAT THE TRAINING AND TEST IMAGES ARE OF EQUAL
    /// SIZE.** (caps-lock, because I got so many mails asking for this). You have to make sure your
    /// input data has the correct shape, else a meaningful exception is thrown. Use resize to resize
    /// the images.
    /// *   This model does not support updating.
    ///
    /// ### Model internal data:
    ///
    /// *   num_components see EigenFaceRecognizer::create.
    /// *   threshold see EigenFaceRecognizer::create.
    /// *   eigenvalues The eigenvalues for this Principal Component Analysis (ordered descending).
    /// *   eigenvectors The eigenvectors for this Principal Component Analysis (ordered by their
    /// eigenvalue).
    /// *   mean The sample mean calculated from the training data.
    /// *   projections The projections of the training data.
    /// *   labels The threshold applied in the prediction. If the distance to the nearest neighbor is
    /// larger than the threshold, this method returns -1.
    ///
    /// ## C++ default parameters
    /// * num_components: 0
    /// * threshold: DBL_MAX
    pub fn create(num_components: i32, threshold: f64) -> Result<types::PtrOfEigenFaceRecognizer> {
        unsafe { sys::cv_face_EigenFaceRecognizer_create_int_double(num_components, threshold) }.into_result().map(|ptr| types::PtrOfEigenFaceRecognizer { ptr })
    }
    
}

// Generating impl for trait crate::face::FaceRecognizer
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
/// start and you can get the list of registered algorithms and create instance of a particular
/// algorithm by its name (see Algorithm::create). If you plan to add your own algorithms, it is
/// good practice to add a unique prefix to your algorithms to distinguish them from other
/// algorithms.
/// *   Setting/Retrieving algorithm parameters by name. If you used video capturing functionality from
/// OpenCV highgui module, you are probably familar with cv::cvSetCaptureProperty,
/// ocvcvGetCaptureProperty, VideoCapture::set and VideoCapture::get. Algorithm provides similar
/// method where instead of integer id's you specify the parameter names as text Strings. See
/// Algorithm::set and Algorithm::get for details.
/// *   Reading and writing parameters from/to XML or YAML files. Every Algorithm derivative can store
/// all its parameters and then read them back. There is no need to re-implement it each time.
///
/// Moreover every FaceRecognizer supports the:
///
/// *   **Training** of a FaceRecognizer with FaceRecognizer::train on a given set of images (your face
/// database!).
/// *   **Prediction** of a given sample image, that means a face. The image is given as a Mat.
/// *   **Loading/Saving** the model state from/to a given XML or YAML.
/// *   **Setting/Getting labels info**, that is stored as a string. String labels info is useful for
/// keeping names of the recognized people.
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
pub trait FaceRecognizer: core::AlgorithmTrait {
    fn as_raw_FaceRecognizer(&self) -> *mut c_void;
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
    fn train(&mut self, src: &dyn core::ToInputArray, labels: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(src);
        input_array_arg!(labels);
        unsafe { sys::cv_face_FaceRecognizer_train__InputArray__InputArray(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray()) }.into_result()
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
        unsafe { sys::cv_face_FaceRecognizer_update__InputArray__InputArray(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), labels.as_raw__InputArray()) }.into_result()
    }
    
    fn predict(&self, src: &dyn core::ToInputArray) -> Result<i32> {
        input_array_arg!(src);
        unsafe { sys::cv_face_FaceRecognizer_predict_const__InputArray(self.as_raw_FaceRecognizer(), src.as_raw__InputArray()) }.into_result()
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
    fn predict_1(&self, src: &dyn core::ToInputArray, label: &mut i32, confidence: &mut f64) -> Result<()> {
        input_array_arg!(src);
        unsafe { sys::cv_face_FaceRecognizer_predict_const__InputArray_int_double(self.as_raw_FaceRecognizer(), src.as_raw__InputArray(), label, confidence) }.into_result()
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
        string_arg!(filename);
        unsafe { sys::cv_face_FaceRecognizer_write_const_String(self.as_raw_FaceRecognizer(), filename.as_ptr()) }.into_result()
    }
    
    /// Loads a FaceRecognizer and its model state.
    ///
    /// Loads a persisted model and state from a given XML or YAML file . Every FaceRecognizer has to
    /// overwrite FaceRecognizer::load(FileStorage& fs) to enable loading the model state.
    /// FaceRecognizer::load(FileStorage& fs) in turn gets called by
    /// FaceRecognizer::load(const String& filename), to ease saving a model.
    fn read(&mut self, filename: &str) -> Result<()> {
        string_arg!(filename);
        unsafe { sys::cv_face_FaceRecognizer_read_String(self.as_raw_FaceRecognizer(), filename.as_ptr()) }.into_result()
    }
    
    /// Saves this model to a given FileStorage.
    /// ## Parameters
    /// * fs: The FileStorage to store this FaceRecognizer to.
    fn write_1(&self, fs: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_face_FaceRecognizer_write_const_FileStorage(self.as_raw_FaceRecognizer(), fs.as_raw_FileStorage()) }.into_result()
    }
    
    fn read_1(&mut self, _fn: &core::FileNode) -> Result<()> {
        unsafe { sys::cv_face_FaceRecognizer_read_FileNode(self.as_raw_FaceRecognizer(), _fn.as_raw_FileNode()) }.into_result()
    }
    
    fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_face_FaceRecognizer_empty_const(self.as_raw_FaceRecognizer()) }.into_result()
    }
    
    /// Sets string info for the specified model's label.
    ///
    /// The string info is replaced by the provided value if it was set before for the specified label.
    fn set_label_info(&mut self, label: i32, str_info: &str) -> Result<()> {
        string_arg!(str_info);
        unsafe { sys::cv_face_FaceRecognizer_setLabelInfo_int_String(self.as_raw_FaceRecognizer(), label, str_info.as_ptr()) }.into_result()
    }
    
    /// Gets string information by label.
    ///
    /// If an unknown label id is provided or there is no label information associated with the specified
    /// label id the method returns an empty string.
    fn get_label_info(&self, label: i32) -> Result<String> {
        unsafe { sys::cv_face_FaceRecognizer_getLabelInfo_const_int(self.as_raw_FaceRecognizer(), label) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Gets vector of labels by string.
    ///
    /// The function searches for the labels containing the specified sub-string in the associated string
    /// info.
    fn get_labels_by_string(&self, str: &str) -> Result<types::VectorOfint> {
        string_arg!(str);
        unsafe { sys::cv_face_FaceRecognizer_getLabelsByString_const_String(self.as_raw_FaceRecognizer(), str.as_ptr()) }.into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
    /// threshold parameter accessor - required for default BestMinDist collector
    fn get_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_face_FaceRecognizer_getThreshold_const(self.as_raw_FaceRecognizer()) }.into_result()
    }
    
    /// Sets threshold of model
    fn set_threshold(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_face_FaceRecognizer_setThreshold_double(self.as_raw_FaceRecognizer(), val) }.into_result()
    }
    
}

// Generating impl for trait crate::face::Facemark
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
    fn as_raw_Facemark(&self) -> *mut c_void;
    /// A function to load the trained model before the fitting process.
    /// ## Parameters
    /// * model: A string represent the filename of a trained model.
    ///
    /// <B>Example of usage</B>
    /// ```ignore
    /// facemark->loadModel("../data/lbf.model");
    /// ```
    fn load_model(&mut self, model: &str) -> Result<()> {
        string_arg!(mut model);
        unsafe { sys::cv_face_Facemark_loadModel_String(self.as_raw_Facemark(), model.as_ptr() as _) }.into_result()
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
    fn fit(&mut self, image: &dyn core::ToInputArray, faces: &dyn core::ToInputArray, landmarks: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(image);
        input_array_arg!(faces);
        output_array_arg!(landmarks);
        unsafe { sys::cv_face_Facemark_fit__InputArray__InputArray__OutputArray(self.as_raw_Facemark(), image.as_raw__InputArray(), faces.as_raw__InputArray(), landmarks.as_raw__OutputArray()) }.into_result()
    }
    
}

// Generating impl for trait crate::face::FacemarkAAM
pub trait FacemarkAAM: crate::face::FacemarkTrain {
    fn as_raw_FacemarkAAM(&self) -> *mut c_void;
    /// overload with additional Config structures
    fn fit_config(&mut self, image: &dyn core::ToInputArray, roi: &dyn core::ToInputArray, _landmarks: &mut dyn core::ToOutputArray, runtime_params: &types::VectorOfConfig) -> Result<bool> {
        input_array_arg!(image);
        input_array_arg!(roi);
        output_array_arg!(_landmarks);
        unsafe { sys::cv_face_FacemarkAAM_fitConfig__InputArray__InputArray__OutputArray_VectorOfConfig(self.as_raw_FacemarkAAM(), image.as_raw__InputArray(), roi.as_raw__InputArray(), _landmarks.as_raw__OutputArray(), runtime_params.as_raw_VectorOfConfig()) }.into_result()
    }
    
}

impl dyn FacemarkAAM + '_ {
    /// initializer
    ///
    /// ## C++ default parameters
    /// * parameters: FacemarkAAM::Params()
    pub fn create(parameters: &crate::face::FacemarkAAM_Params) -> Result<types::PtrOfFacemarkAAM> {
        unsafe { sys::cv_face_FacemarkAAM_create_Params(parameters.as_raw_FacemarkAAM_Params()) }.into_result().map(|ptr| types::PtrOfFacemarkAAM { ptr })
    }
    
}

// boxed class cv::face::FacemarkAAM::Config
/// \brief Optional parameter for fitting process.
pub struct FacemarkAAM_Config {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkAAM_Config {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkAAM_Config_delete(self.ptr) };
    }
}

impl FacemarkAAM_Config {
    #[inline(always)] pub fn as_raw_FacemarkAAM_Config(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkAAM_Config {}

impl FacemarkAAM_Config {
    ///
    /// ## C++ default parameters
    /// * rot: Mat::eye(2,2,CV_32F)
    /// * trans: Point2f(0.0f, 0.0f)
    /// * scaling: 1.0f
    /// * scale_id: 0
    pub fn new(rot: &core::Mat, trans: core::Point2f, scaling: f32, scale_id: i32) -> Result<crate::face::FacemarkAAM_Config> {
        unsafe { sys::cv_face_FacemarkAAM_Config_Config_Mat_Point2f_float_int(rot.as_raw_Mat(), trans, scaling, scale_id) }.into_result().map(|ptr| crate::face::FacemarkAAM_Config { ptr })
    }
    
}

// boxed class cv::face::FacemarkAAM::Data
/// \brief Data container for the facemark::getData function
pub struct FacemarkAAM_Data {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkAAM_Data {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkAAM_Data_delete(self.ptr) };
    }
}

impl FacemarkAAM_Data {
    #[inline(always)] pub fn as_raw_FacemarkAAM_Data(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkAAM_Data {}

// boxed class cv::face::FacemarkAAM::Model
/// \brief The model of AAM Algorithm
pub struct FacemarkAAM_Model {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkAAM_Model {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkAAM_Model_delete(self.ptr) };
    }
}

impl FacemarkAAM_Model {
    #[inline(always)] pub fn as_raw_FacemarkAAM_Model(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkAAM_Model {}

// boxed class cv::face::FacemarkAAM::Model::Texture
pub struct FacemarkAAM_Model_Texture {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkAAM_Model_Texture {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkAAM_Model_Texture_delete(self.ptr) };
    }
}

impl FacemarkAAM_Model_Texture {
    #[inline(always)] pub fn as_raw_FacemarkAAM_Model_Texture(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkAAM_Model_Texture {}

// boxed class cv::face::FacemarkAAM::Params
pub struct FacemarkAAM_Params {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkAAM_Params {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkAAM_Params_delete(self.ptr) };
    }
}

impl FacemarkAAM_Params {
    #[inline(always)] pub fn as_raw_FacemarkAAM_Params(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkAAM_Params {}

impl FacemarkAAM_Params {
    /// \brief Constructor
    pub fn default() -> Result<crate::face::FacemarkAAM_Params> {
        unsafe { sys::cv_face_FacemarkAAM_Params_Params() }.into_result().map(|ptr| crate::face::FacemarkAAM_Params { ptr })
    }
    
    /// \brief Read parameters from file, currently unused
    pub fn read(&mut self, unnamed_arg: &core::FileNode) -> Result<()> {
        unsafe { sys::cv_face_FacemarkAAM_Params_read_FileNode(self.as_raw_FacemarkAAM_Params(), unnamed_arg.as_raw_FileNode()) }.into_result()
    }
    
    /// \brief Read parameters from file, currently unused
    pub fn write(&self, unnamed_arg: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_face_FacemarkAAM_Params_write_const_FileStorage(self.as_raw_FacemarkAAM_Params(), unnamed_arg.as_raw_FileStorage()) }.into_result()
    }
    
}

// Generating impl for trait crate::face::FacemarkKazemi
pub trait FacemarkKazemi: crate::face::Facemark {
    fn as_raw_FacemarkKazemi(&self) -> *mut c_void;
    /// This function is used to train the model using gradient boosting to get a cascade of regressors
    /// *which can then be used to predict shape.
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
    fn training(&mut self, images: &mut types::VectorOfMat, landmarks: &mut types::VectorOfVectorOfPoint2f, configfile: &str, scale: core::Size, model_filename: &str) -> Result<bool> {
        string_arg!(mut configfile);
        string_arg!(mut model_filename);
        unsafe { sys::cv_face_FacemarkKazemi_training_VectorOfMat_VectorOfVectorOfPoint2f_std_string_Size_std_string(self.as_raw_FacemarkKazemi(), images.as_raw_VectorOfMat(), landmarks.as_raw_VectorOfVectorOfPoint2f(), configfile.as_ptr() as _, scale, model_filename.as_ptr() as _) }.into_result()
    }
    
    fn get_faces(&mut self, image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(image);
        output_array_arg!(faces);
        unsafe { sys::cv_face_FacemarkKazemi_getFaces__InputArray__OutputArray(self.as_raw_FacemarkKazemi(), image.as_raw__InputArray(), faces.as_raw__OutputArray()) }.into_result()
    }
    
}

impl dyn FacemarkKazemi + '_ {
    ///
    /// ## C++ default parameters
    /// * parameters: FacemarkKazemi::Params()
    pub fn create(parameters: &crate::face::FacemarkKazemi_Params) -> Result<types::PtrOfFacemarkKazemi> {
        unsafe { sys::cv_face_FacemarkKazemi_create_Params(parameters.as_raw_FacemarkKazemi_Params()) }.into_result().map(|ptr| types::PtrOfFacemarkKazemi { ptr })
    }
    
}

// boxed class cv::face::FacemarkKazemi::Params
pub struct FacemarkKazemi_Params {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkKazemi_Params {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkKazemi_Params_delete(self.ptr) };
    }
}

impl FacemarkKazemi_Params {
    #[inline(always)] pub fn as_raw_FacemarkKazemi_Params(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkKazemi_Params {}

impl FacemarkKazemi_Params {
    /// \brief Constructor
    pub fn default() -> Result<crate::face::FacemarkKazemi_Params> {
        unsafe { sys::cv_face_FacemarkKazemi_Params_Params() }.into_result().map(|ptr| crate::face::FacemarkKazemi_Params { ptr })
    }
    
}

// boxed class cv::face::FacemarkLBF
pub struct FacemarkLBF {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkLBF {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkLBF_delete(self.ptr) };
    }
}

impl FacemarkLBF {
    #[inline(always)] pub fn as_raw_FacemarkLBF(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkLBF {}

impl core::AlgorithmTrait for FacemarkLBF {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::face::Facemark for FacemarkLBF {
    #[inline(always)] fn as_raw_Facemark(&self) -> *mut c_void { self.ptr }
}

impl crate::face::FacemarkTrain for FacemarkLBF {
    #[inline(always)] fn as_raw_FacemarkTrain(&self) -> *mut c_void { self.ptr }
}

impl FacemarkLBF {
    ///
    /// ## C++ default parameters
    /// * parameters: FacemarkLBF::Params()
    pub fn create(parameters: &crate::face::FacemarkLBF_Params) -> Result<types::PtrOfFacemarkLBF> {
        unsafe { sys::cv_face_FacemarkLBF_create_Params(parameters.as_raw_FacemarkLBF_Params()) }.into_result().map(|ptr| types::PtrOfFacemarkLBF { ptr })
    }
    
}

// boxed class cv::face::FacemarkLBF::BBox
pub struct FacemarkLBF_BBox {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkLBF_BBox {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkLBF_BBox_delete(self.ptr) };
    }
}

impl FacemarkLBF_BBox {
    #[inline(always)] pub fn as_raw_FacemarkLBF_BBox(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkLBF_BBox {}

impl FacemarkLBF_BBox {
    pub fn default() -> Result<crate::face::FacemarkLBF_BBox> {
        unsafe { sys::cv_face_FacemarkLBF_BBox_BBox() }.into_result().map(|ptr| crate::face::FacemarkLBF_BBox { ptr })
    }
    
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Result<crate::face::FacemarkLBF_BBox> {
        unsafe { sys::cv_face_FacemarkLBF_BBox_BBox_double_double_double_double(x, y, w, h) }.into_result().map(|ptr| crate::face::FacemarkLBF_BBox { ptr })
    }
    
    pub fn project(&self, shape: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_face_FacemarkLBF_BBox_project_const_Mat(self.as_raw_FacemarkLBF_BBox(), shape.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    pub fn reproject(&self, shape: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_face_FacemarkLBF_BBox_reproject_const_Mat(self.as_raw_FacemarkLBF_BBox(), shape.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

// boxed class cv::face::FacemarkLBF::Params
pub struct FacemarkLBF_Params {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FacemarkLBF_Params {
    fn drop(&mut self) {
        unsafe { sys::cv_FacemarkLBF_Params_delete(self.ptr) };
    }
}

impl FacemarkLBF_Params {
    #[inline(always)] pub fn as_raw_FacemarkLBF_Params(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FacemarkLBF_Params {}

impl FacemarkLBF_Params {
    /// \brief Constructor
    pub fn default() -> Result<crate::face::FacemarkLBF_Params> {
        unsafe { sys::cv_face_FacemarkLBF_Params_Params() }.into_result().map(|ptr| crate::face::FacemarkLBF_Params { ptr })
    }
    
    pub fn read(&mut self, unnamed_arg: &core::FileNode) -> Result<()> {
        unsafe { sys::cv_face_FacemarkLBF_Params_read_FileNode(self.as_raw_FacemarkLBF_Params(), unnamed_arg.as_raw_FileNode()) }.into_result()
    }
    
    pub fn write(&self, unnamed_arg: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_face_FacemarkLBF_Params_write_const_FileStorage(self.as_raw_FacemarkLBF_Params(), unnamed_arg.as_raw_FileStorage()) }.into_result()
    }
    
}

// Generating impl for trait crate::face::FacemarkTrain
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
/// The facemark algorithms are designed to fit the facial points into a face.
/// Therefore, the face information should be provided to the facemark algorithm.
/// Some algorithms might provides a default face recognition function.
/// However, the users might prefer to use their own face detector to obtains the best possible detection result.
/// - (Non-mandatory) Training the model for a specific algorithm using FacemarkTrain::training.
/// In this case, the model should be automatically saved by the algorithm.
/// If the user already have a trained model, then this part can be omitted.
/// - Load the trained model using Facemark::loadModel.
/// - Perform the fitting via the Facemark::fit.
pub trait FacemarkTrain: crate::face::Facemark {
    fn as_raw_FacemarkTrain(&self) -> *mut c_void;
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
    /// image = imread(images_train[i].c_str());
    /// loadFacePoints(landmarks_train[i],facial_points);
    /// facemark->addTrainingSample(image, facial_points);
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
    fn add_training_sample(&mut self, image: &dyn core::ToInputArray, landmarks: &dyn core::ToInputArray) -> Result<bool> {
        input_array_arg!(image);
        input_array_arg!(landmarks);
        unsafe { sys::cv_face_FacemarkTrain_addTrainingSample__InputArray__InputArray(self.as_raw_FacemarkTrain(), image.as_raw__InputArray(), landmarks.as_raw__InputArray()) }.into_result()
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
    /// ## C++ default parameters
    /// * parameters: 0
    fn training(&mut self, parameters: &mut c_void) -> Result<()> {
        unsafe { sys::cv_face_FacemarkTrain_training_void_X(self.as_raw_FacemarkTrain(), parameters) }.into_result()
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
    /// cv::rectangle(img, faces[j], cv::Scalar(255,0,255));
    /// }
    /// ```
    fn get_faces(&mut self, image: &dyn core::ToInputArray, faces: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(image);
        output_array_arg!(faces);
        unsafe { sys::cv_face_FacemarkTrain_getFaces__InputArray__OutputArray(self.as_raw_FacemarkTrain(), image.as_raw__InputArray(), faces.as_raw__OutputArray()) }.into_result()
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
    /// ## C++ default parameters
    /// * items: 0
    fn get_data(&mut self, items: &mut c_void) -> Result<bool> {
        unsafe { sys::cv_face_FacemarkTrain_getData_void_X(self.as_raw_FacemarkTrain(), items) }.into_result()
    }
    
}

// boxed class cv::face::FisherFaceRecognizer
pub struct FisherFaceRecognizer {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FisherFaceRecognizer {
    fn drop(&mut self) {
        unsafe { sys::cv_FisherFaceRecognizer_delete(self.ptr) };
    }
}

impl FisherFaceRecognizer {
    #[inline(always)] pub fn as_raw_FisherFaceRecognizer(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FisherFaceRecognizer {}

impl core::AlgorithmTrait for FisherFaceRecognizer {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::face::BasicFaceRecognizerTrait for FisherFaceRecognizer {
    #[inline(always)] fn as_raw_BasicFaceRecognizer(&self) -> *mut c_void { self.ptr }
}

impl crate::face::FaceRecognizer for FisherFaceRecognizer {
    #[inline(always)] fn as_raw_FaceRecognizer(&self) -> *mut c_void { self.ptr }
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
    /// color spaces.
    /// *   **THE FISHERFACES METHOD MAKES THE ASSUMPTION, THAT THE TRAINING AND TEST IMAGES ARE OF EQUAL
    /// SIZE.** (caps-lock, because I got so many mails asking for this). You have to make sure your
    /// input data has the correct shape, else a meaningful exception is thrown. Use resize to resize
    /// the images.
    /// *   This model does not support updating.
    ///
    /// ### Model internal data:
    ///
    /// *   num_components see FisherFaceRecognizer::create.
    /// *   threshold see FisherFaceRecognizer::create.
    /// *   eigenvalues The eigenvalues for this Linear Discriminant Analysis (ordered descending).
    /// *   eigenvectors The eigenvectors for this Linear Discriminant Analysis (ordered by their
    /// eigenvalue).
    /// *   mean The sample mean calculated from the training data.
    /// *   projections The projections of the training data.
    /// *   labels The labels corresponding to the projections.
    ///
    /// ## C++ default parameters
    /// * num_components: 0
    /// * threshold: DBL_MAX
    pub fn create(num_components: i32, threshold: f64) -> Result<types::PtrOfFisherFaceRecognizer> {
        unsafe { sys::cv_face_FisherFaceRecognizer_create_int_double(num_components, threshold) }.into_result().map(|ptr| types::PtrOfFisherFaceRecognizer { ptr })
    }
    
}

// Generating impl for trait crate::face::LBPHFaceRecognizer
pub trait LBPHFaceRecognizer: crate::face::FaceRecognizer {
    fn as_raw_LBPHFaceRecognizer(&self) -> *mut c_void;
    /// @see setGridX
    fn get_grid_x(&self) -> Result<i32> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_getGridX_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
    }
    
    /// @copybrief getGridX @see getGridX
    fn set_grid_x(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_setGridX_int(self.as_raw_LBPHFaceRecognizer(), val) }.into_result()
    }
    
    /// @see setGridY
    fn get_grid_y(&self) -> Result<i32> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_getGridY_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
    }
    
    /// @copybrief getGridY @see getGridY
    fn set_grid_y(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_setGridY_int(self.as_raw_LBPHFaceRecognizer(), val) }.into_result()
    }
    
    /// @see setRadius
    fn get_radius(&self) -> Result<i32> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_getRadius_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
    }
    
    /// @copybrief getRadius @see getRadius
    fn set_radius(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_setRadius_int(self.as_raw_LBPHFaceRecognizer(), val) }.into_result()
    }
    
    /// @see setNeighbors
    fn get_neighbors(&self) -> Result<i32> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_getNeighbors_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
    }
    
    /// @copybrief getNeighbors @see getNeighbors
    fn set_neighbors(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_setNeighbors_int(self.as_raw_LBPHFaceRecognizer(), val) }.into_result()
    }
    
    /// @see setThreshold
    fn get_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_getThreshold_const(self.as_raw_LBPHFaceRecognizer()) }.into_result()
    }
    
    /// @copybrief getThreshold @see getThreshold
    fn set_threshold(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_setThreshold_double(self.as_raw_LBPHFaceRecognizer(), val) }.into_result()
    }
    
    fn get_histograms(&self) -> Result<types::VectorOfMat> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_getHistograms_const(self.as_raw_LBPHFaceRecognizer()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
    }
    
    fn get_labels(&self) -> Result<core::Mat> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_getLabels_const(self.as_raw_LBPHFaceRecognizer()) }.into_result().map(|ptr| core::Mat { ptr })
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
    /// grayscale images, use cvtColor to convert between the color spaces.
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
    /// none was given).
    /// *   labels Labels corresponding to the calculated Local Binary Patterns Histograms.
    ///
    /// ## C++ default parameters
    /// * radius: 1
    /// * neighbors: 8
    /// * grid_x: 8
    /// * grid_y: 8
    /// * threshold: DBL_MAX
    pub fn create(radius: i32, neighbors: i32, grid_x: i32, grid_y: i32, threshold: f64) -> Result<types::PtrOfLBPHFaceRecognizer> {
        unsafe { sys::cv_face_LBPHFaceRecognizer_create_int_int_int_int_double(radius, neighbors, grid_x, grid_y, threshold) }.into_result().map(|ptr| types::PtrOfLBPHFaceRecognizer { ptr })
    }
    
}

// Generating impl for trait crate::face::MACE
/// Minimum Average Correlation Energy Filter
/// useful for authentication with (cancellable) biometrical features.
/// (does not need many positives to train (10-50), and no negatives at all, also robust to noise/salting)
///
/// see also: [Savvides04](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_Savvides04)
///
/// this implementation is largely based on: https://code.google.com/archive/p/pam-face-authentication (GSOC 2009)
///
/// use it like:
/// ```ignore
///
/// Ptr<face::MACE> mace = face::MACE::create(64);
///
/// vector<Mat> pos_images = ...
/// mace->train(pos_images);
///
/// Mat query = ...
/// bool same = mace->same(query);
///
/// ```
///
///
/// you can also use two-factor authentication, with an additional passphrase:
///
/// ```ignore
/// String owners_passphrase = "ilikehotdogs";
/// Ptr<face::MACE> mace = face::MACE::create(64);
/// mace->salt(owners_passphrase);
/// vector<Mat> pos_images = ...
/// mace->train(pos_images);
///
/// // now, users have to give a valid passphrase, along with the image:
/// Mat query = ...
/// cout << "enter passphrase: ";
/// string pass;
/// getline(cin, pass);
/// mace->salt(pass);
/// bool same = mace->same(query);
/// ```
///
///
/// save/load your model:
/// ```ignore
/// Ptr<face::MACE> mace = face::MACE::create(64);
/// mace->train(pos_images);
/// mace->save("my_mace.xml");
///
/// // later:
/// Ptr<MACE> reloaded = MACE::load("my_mace.xml");
/// reloaded->same(some_image);
/// ```
pub trait MACE: core::AlgorithmTrait {
    fn as_raw_MACE(&self) -> *mut c_void;
    /// optionally encrypt images with random convolution
    /// ## Parameters
    /// * passphrase: a crc64 random seed will get generated from this
    fn salt(&mut self, passphrase: &str) -> Result<()> {
        string_arg!(passphrase);
        unsafe { sys::cv_face_MACE_salt_String(self.as_raw_MACE(), passphrase.as_ptr()) }.into_result()
    }
    
}

// Generating impl for trait crate::face::PredictCollector
/// Abstract base class for all strategies of prediction result handling
pub trait PredictCollector {
    fn as_raw_PredictCollector(&self) -> *mut c_void;
    /// Interface method called by face recognizer before results processing
    /// ## Parameters
    /// * size: total size of prediction evaluation that recognizer could perform
    fn init(&mut self, size: size_t) -> Result<()> {
        unsafe { sys::cv_face_PredictCollector_init_size_t(self.as_raw_PredictCollector(), size) }.into_result()
    }
    
    /// Interface method called by face recognizer for each result
    /// ## Parameters
    /// * label: current prediction label
    /// * dist: current prediction distance (confidence)
    fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
        unsafe { sys::cv_face_PredictCollector_collect_int_double(self.as_raw_PredictCollector(), label, dist) }.into_result()
    }
    
}

// boxed class cv::face::StandardCollector
/// Default predict collector
///
/// Trace minimal distance with treshhold checking (that is default behavior for most predict logic)
pub struct StandardCollector {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for StandardCollector {
    fn drop(&mut self) {
        unsafe { sys::cv_StandardCollector_delete(self.ptr) };
    }
}

impl StandardCollector {
    #[inline(always)] pub fn as_raw_StandardCollector(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for StandardCollector {}

impl crate::face::PredictCollector for StandardCollector {
    #[inline(always)] fn as_raw_PredictCollector(&self) -> *mut c_void { self.ptr }
}

impl StandardCollector {
    /// Constructor
    /// ## Parameters
    /// * threshold_: set threshold
    ///
    /// ## C++ default parameters
    /// * threshold_: DBL_MAX
    pub fn new(threshold_: f64) -> Result<crate::face::StandardCollector> {
        unsafe { sys::cv_face_StandardCollector_StandardCollector_double(threshold_) }.into_result().map(|ptr| crate::face::StandardCollector { ptr })
    }
    
    /// overloaded interface method
    pub fn init(&mut self, size: size_t) -> Result<()> {
        unsafe { sys::cv_face_StandardCollector_init_size_t(self.as_raw_StandardCollector(), size) }.into_result()
    }
    
    /// overloaded interface method
    pub fn collect(&mut self, label: i32, dist: f64) -> Result<bool> {
        unsafe { sys::cv_face_StandardCollector_collect_int_double(self.as_raw_StandardCollector(), label, dist) }.into_result()
    }
    
    /// Returns label with minimal distance
    pub fn get_min_label(&self) -> Result<i32> {
        unsafe { sys::cv_face_StandardCollector_getMinLabel_const(self.as_raw_StandardCollector()) }.into_result()
    }
    
    /// Returns minimal distance value
    pub fn get_min_dist(&self) -> Result<f64> {
        unsafe { sys::cv_face_StandardCollector_getMinDist_const(self.as_raw_StandardCollector()) }.into_result()
    }
    
    /// Static constructor
    /// ## Parameters
    /// * threshold: set threshold
    ///
    /// ## C++ default parameters
    /// * threshold: DBL_MAX
    pub fn create(threshold: f64) -> Result<types::PtrOfStandardCollector> {
        unsafe { sys::cv_face_StandardCollector_create_double(threshold) }.into_result().map(|ptr| types::PtrOfStandardCollector { ptr })
    }
    
}

// boxed class cv::face::StandardCollector::PredictResult
pub struct StandardCollector_PredictResult {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for StandardCollector_PredictResult {
    fn drop(&mut self) {
        unsafe { sys::cv_StandardCollector_PredictResult_delete(self.ptr) };
    }
}

impl StandardCollector_PredictResult {
    #[inline(always)] pub fn as_raw_StandardCollector_PredictResult(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for StandardCollector_PredictResult {}

impl StandardCollector_PredictResult {
    ///
    /// ## C++ default parameters
    /// * label_: -1
    /// * distance_: DBL_MAX
    pub fn new(label_: i32, distance_: f64) -> Result<crate::face::StandardCollector_PredictResult> {
        unsafe { sys::cv_face_StandardCollector_PredictResult_PredictResult_int_double(label_, distance_) }.into_result().map(|ptr| crate::face::StandardCollector_PredictResult { ptr })
    }
    
}


//! # Face Recognition
//!
//! - @ref face_changelog
//! - @ref tutorial_face_main
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};


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
/// *   num_components see createEigenFaceRecognizer.
/// *   threshold see createEigenFaceRecognizer.
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
pub fn create_eigen_face_recognizer(num_components: i32, threshold: f64) -> Result<types::PtrOfBasicFaceRecognizer> {
    unsafe { sys::cv_face_createEigenFaceRecognizer_int_double(num_components, threshold) }.into_result().map(|ptr| types::PtrOfBasicFaceRecognizer { ptr })
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
/// color spaces.
/// *   **THE FISHERFACES METHOD MAKES THE ASSUMPTION, THAT THE TRAINING AND TEST IMAGES ARE OF EQUAL
/// SIZE.** (caps-lock, because I got so many mails asking for this). You have to make sure your
/// input data has the correct shape, else a meaningful exception is thrown. Use resize to resize
/// the images.
/// *   This model does not support updating.
///
/// ### Model internal data:
///
/// *   num_components see createFisherFaceRecognizer.
/// *   threshold see createFisherFaceRecognizer.
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
pub fn create_fisher_face_recognizer(num_components: i32, threshold: f64) -> Result<types::PtrOfBasicFaceRecognizer> {
    unsafe { sys::cv_face_createFisherFaceRecognizer_int_double(num_components, threshold) }.into_result().map(|ptr| types::PtrOfBasicFaceRecognizer { ptr })
}

/// ## Parameters
/// * radius: The radius used for building the Circular Local Binary Pattern. The greater the
/// radius, the
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
/// *   radius see createLBPHFaceRecognizer.
/// *   neighbors see createLBPHFaceRecognizer.
/// *   grid_x see createLBPHFaceRecognizer.
/// *   grid_y see createLBPHFaceRecognizer.
/// *   threshold see createLBPHFaceRecognizer.
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
pub fn create_lbph_face_recognizer(radius: i32, neighbors: i32, grid_x: i32, grid_y: i32, threshold: f64) -> Result<types::PtrOfLBPHFaceRecognizer> {
    unsafe { sys::cv_face_createLBPHFaceRecognizer_int_int_int_int_double(radius, neighbors, grid_x, grid_y, threshold) }.into_result().map(|ptr| types::PtrOfLBPHFaceRecognizer { ptr })
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

// Generating impl for trait crate::face::BasicFaceRecognizer
pub trait BasicFaceRecognizer: crate::face::FaceRecognizer {
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
/// *   So called “virtual constructor”. That is, each Algorithm derivative is registered at program
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
/// // create the concrete implementation with the appropiate parameters:
/// Ptr<FaceRecognizer> model = createEigenFaceRecognizer(num_components, threshold);
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
/// Mat img = imread("person1/3.jpg", CV_LOAD_IMAGE_GRAYSCALE);
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
/// Ptr<FaceRecognizer> model = createEigenFaceRecognizer();
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
    /// or a
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
    /// // images for first person
    /// images.push_back(imread("person0/0.jpg", CV_LOAD_IMAGE_GRAYSCALE)); labels.push_back(0);
    /// images.push_back(imread("person0/1.jpg", CV_LOAD_IMAGE_GRAYSCALE)); labels.push_back(0);
    /// images.push_back(imread("person0/2.jpg", CV_LOAD_IMAGE_GRAYSCALE)); labels.push_back(0);
    /// // images for second person
    /// images.push_back(imread("person1/0.jpg", CV_LOAD_IMAGE_GRAYSCALE)); labels.push_back(1);
    /// images.push_back(imread("person1/1.jpg", CV_LOAD_IMAGE_GRAYSCALE)); labels.push_back(1);
    /// images.push_back(imread("person1/2.jpg", CV_LOAD_IMAGE_GRAYSCALE)); labels.push_back(1);
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
    /// Ptr<FaceRecognizer> model =  createFisherFaceRecognizer();
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
    /// a
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
    /// Ptr<FaceRecognizer> model =  createLBPHFaceRecognizer();
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
    /// Calling update on an Eigenfaces model (see createEigenFaceRecognizer), which doesn't support
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
    /// Mat img = imread("person1/3.jpg", CV_LOAD_IMAGE_GRAYSCALE);
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
    /// Mat img = imread("person1/3.jpg", CV_LOAD_IMAGE_GRAYSCALE);
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
    fn save(&self, filename: &str) -> Result<()> {
        string_arg!(filename);
        unsafe { sys::cv_face_FaceRecognizer_save_const_String(self.as_raw_FaceRecognizer(), filename.as_ptr()) }.into_result()
    }
    
    /// Loads a FaceRecognizer and its model state.
    ///
    /// Loads a persisted model and state from a given XML or YAML file . Every FaceRecognizer has to
    /// overwrite FaceRecognizer::load(FileStorage& fs) to enable loading the model state.
    /// FaceRecognizer::load(FileStorage& fs) in turn gets called by
    /// FaceRecognizer::load(const String& filename), to ease saving a model.
    fn load(&mut self, filename: &str) -> Result<()> {
        string_arg!(filename);
        unsafe { sys::cv_face_FaceRecognizer_load_String(self.as_raw_FaceRecognizer(), filename.as_ptr()) }.into_result()
    }
    
    /// Saves this model to a given FileStorage.
    /// ## Parameters
    /// * fs: The FileStorage to store this FaceRecognizer to.
    fn save_1(&self, fs: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_face_FaceRecognizer_save_const_FileStorage(self.as_raw_FaceRecognizer(), fs.as_raw_FileStorage()) }.into_result()
    }
    
    fn load_1(&mut self, fs: &core::FileStorage) -> Result<()> {
        unsafe { sys::cv_face_FaceRecognizer_load_FileStorage(self.as_raw_FaceRecognizer(), fs.as_raw_FileStorage()) }.into_result()
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


//! # Biologically inspired vision models and derivated tools
//!
//! The module provides biological visual systems models (human visual system and others). It also
//! provides derivated objects that take advantage of those bio-inspired models.
//!
//! @ref bioinspired_retina
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};

/// standard bayer sampling
pub const RETINA_COLOR_BAYER: i32 = 2;
/// color sampling is RGBRGBRGB..., line 2 BRGBRGBRG..., line 3, GBRGBRGBR...
pub const RETINA_COLOR_DIAGONAL: i32 = 1;
/// each pixel position is either R, G or B in a random choice
pub const RETINA_COLOR_RANDOM: i32 = 0;

// Generating impl for trait cv::bioinspired::Retina (trait)
/// class which allows the Gipsa/Listic Labs model to be used with OpenCV.
///
/// This retina model allows spatio-temporal image processing (applied on still images, video sequences).
/// As a summary, these are the retina model properties:
/// - It applies a spectral whithening (mid-frequency details enhancement)
/// - high frequency spatio-temporal noise reduction
/// - low frequency luminance to be reduced (luminance range compression)
/// - local logarithmic luminance compression allows details to be enhanced in low light conditions
///
/// USE : this model can be used basically for spatio-temporal video effects but also for :
/// _using the getParvo method output matrix : texture analysiswith enhanced signal to noise ratio and enhanced details robust against input images luminance ranges
/// _using the getMagno method output matrix : motion analysis also with the previously cited properties
///
/// for more information, reer to the following papers :
/// Benoit A., Caplier A., Durette B., Herault, J., "USING HUMAN VISUAL SYSTEM MODELING FOR BIO-INSPIRED LOW LEVEL IMAGE PROCESSING", Elsevier, Computer Vision and Image Understanding 114 (2010), pp. 758-773, DOI: http://dx.doi.org/10.1016/j.cviu.2010.01.011
/// Vision: Images, Signals and Neural Networks: Models of Neural Processing in Visual Perception (Progress in Neural Processing),By: Jeanny Herault, ISBN: 9814273686. WAPI (Tower ID): 113266891.
///
/// The retina filter includes the research contributions of phd/research collegues from which code has been redrawn by the author :
/// take a look at the retinacolor.hpp module to discover Brice Chaix de Lavarene color mosaicing/demosaicing and the reference paper:
/// B. Chaix de Lavarene, D. Alleysson, B. Durette, J. Herault (2007). "Efficient demosaicing through recursive filtering", IEEE International Conference on Image Processing ICIP 2007
/// take a look at imagelogpolprojection.hpp to discover retina spatial log sampling which originates from Barthelemy Durette phd with Jeanny Herault. A Retina / V1 cortex projection is also proposed and originates from Jeanny's discussions.
/// more informations in the above cited Jeanny Heraults's book.
pub trait Retina: core::Algorithm {
    #[inline(always)] fn as_raw_Retina(&self) -> *mut c_void;
    /// Retreive retina input buffer size
    /// ## Returns
    /// the retina input buffer size
    fn get_input_size(&mut self) -> Result<core::Size> {
        unsafe { sys::cv_bioinspired_Retina_getInputSize(self.as_raw_Retina()) }.into_result()
    }
    
    /// Retreive retina output buffer size that can be different from the input if a spatial log
    /// transformation is applied
    /// ## Returns
    /// the retina output buffer size
    fn get_output_size(&mut self) -> Result<core::Size> {
        unsafe { sys::cv_bioinspired_Retina_getOutputSize(self.as_raw_Retina()) }.into_result()
    }
    
    /// Try to open an XML retina parameters file to adjust current retina instance setup
    ///
    /// - if the xml file does not exist, then default setup is applied
    /// - warning, Exceptions are thrown if read XML file is not valid
    /// ## Parameters
    /// * retinaParameterFile: the parameters filename
    /// * applyDefaultSetupOnFailure: set to true if an error must be thrown on error
    ///
    /// You can retrieve the current parameters structure using the method Retina::getParameters and update
    /// it before running method Retina::setup.
    ///
    /// ## C++ default parameters
    /// * retina_parameter_file: ""
    /// * apply_default_setup_on_failure: true
    fn setup(&mut self, retina_parameter_file: &str, apply_default_setup_on_failure: bool) -> Result<()> {
        string_arg!(mut retina_parameter_file);
        unsafe { sys::cv_bioinspired_Retina_setup_String_bool(self.as_raw_Retina(), retina_parameter_file.as_ptr() as _, apply_default_setup_on_failure) }.into_result()
    }
    
    /// ## Parameters
    /// * newParameters: a parameters structures updated with the new target configuration.
    fn setup_1(&mut self, new_parameters: &crate::bioinspired::RetinaParameters) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_setup_RetinaParameters(self.as_raw_Retina(), new_parameters.as_raw_RetinaParameters()) }.into_result()
    }
    
    /// ## Returns
    /// the current parameters setup
    fn get_parameters(&mut self) -> Result<crate::bioinspired::RetinaParameters> {
        unsafe { sys::cv_bioinspired_Retina_getParameters(self.as_raw_Retina()) }.into_result().map(|ptr| crate::bioinspired::RetinaParameters { ptr })
    }
    
    /// Outputs a string showing the used parameters setup
    /// ## Returns
    /// a string which contains formated parameters information
    fn print_setup(&mut self) -> Result<String> {
        unsafe { sys::cv_bioinspired_Retina_printSetup(self.as_raw_Retina()) }.into_result().map(crate::templ::receive_string)
    }
    
    /// Write xml/yml formated parameters information
    /// ## Parameters
    /// * fs: the filename of the xml file that will be open and writen with formatted parameters
    /// information
    fn write(&self, fs: &str) -> Result<()> {
        string_arg!(mut fs);
        unsafe { sys::cv_bioinspired_Retina_write_const_String(self.as_raw_Retina(), fs.as_ptr() as _) }.into_result()
    }
    
    /// Setup the OPL and IPL parvo channels (see biologocal model)
    ///
    /// OPL is referred as Outer Plexiform Layer of the retina, it allows the spatio-temporal filtering
    /// which withens the spectrum and reduces spatio-temporal noise while attenuating global luminance
    /// (low frequency energy) IPL parvo is the OPL next processing stage, it refers to a part of the
    /// Inner Plexiform layer of the retina, it allows high contours sensitivity in foveal vision. See
    /// reference papers for more informations.
    /// for more informations, please have a look at the paper Benoit A., Caplier A., Durette B., Herault, J., "USING HUMAN VISUAL SYSTEM MODELING FOR BIO-INSPIRED LOW LEVEL IMAGE PROCESSING", Elsevier, Computer Vision and Image Understanding 114 (2010), pp. 758-773, DOI: http://dx.doi.org/10.1016/j.cviu.2010.01.011
    /// ## Parameters
    /// * colorMode: specifies if (true) color is processed of not (false) to then processing gray
    /// level image
    /// * normaliseOutput: specifies if (true) output is rescaled between 0 and 255 of not (false)
    /// * photoreceptorsLocalAdaptationSensitivity: the photoreceptors sensitivity renage is 0-1
    /// (more log compression effect when value increases)
    /// * photoreceptorsTemporalConstant: the time constant of the first order low pass filter of
    /// the photoreceptors, use it to cut high temporal frequencies (noise or fast motion), unit is
    /// frames, typical value is 1 frame
    /// * photoreceptorsSpatialConstant: the spatial constant of the first order low pass filter of
    /// the photoreceptors, use it to cut high spatial frequencies (noise or thick contours), unit is
    /// pixels, typical value is 1 pixel
    /// * horizontalCellsGain: gain of the horizontal cells network, if 0, then the mean value of
    /// the output is zero, if the parameter is near 1, then, the luminance is not filtered and is
    /// still reachable at the output, typicall value is 0
    /// * HcellsTemporalConstant: the time constant of the first order low pass filter of the
    /// horizontal cells, use it to cut low temporal frequencies (local luminance variations), unit is
    /// frames, typical value is 1 frame, as the photoreceptors
    /// * HcellsSpatialConstant: the spatial constant of the first order low pass filter of the
    /// horizontal cells, use it to cut low spatial frequencies (local luminance), unit is pixels,
    /// typical value is 5 pixel, this value is also used for local contrast computing when computing
    /// the local contrast adaptation at the ganglion cells level (Inner Plexiform Layer parvocellular
    /// channel model)
    /// * ganglionCellsSensitivity: the compression strengh of the ganglion cells local adaptation
    /// output, set a value between 0.6 and 1 for best results, a high value increases more the low
    /// value sensitivity... and the output saturates faster, recommended value: 0.7
    ///
    /// ## C++ default parameters
    /// * color_mode: true
    /// * normalise_output: true
    /// * photoreceptors_local_adaptation_sensitivity: 0.7f
    /// * photoreceptors_temporal_constant: 0.5f
    /// * photoreceptors_spatial_constant: 0.53f
    /// * horizontal_cells_gain: 0.f
    /// * hcells_temporal_constant: 1.f
    /// * hcells_spatial_constant: 7.f
    /// * ganglion_cells_sensitivity: 0.7f
    fn setup_op_land_ipl_parvo_channel(&mut self, color_mode: bool, normalise_output: bool, photoreceptors_local_adaptation_sensitivity: f32, photoreceptors_temporal_constant: f32, photoreceptors_spatial_constant: f32, horizontal_cells_gain: f32, hcells_temporal_constant: f32, hcells_spatial_constant: f32, ganglion_cells_sensitivity: f32) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_setupOPLandIPLParvoChannel_bool_bool_float_float_float_float_float_float_float(self.as_raw_Retina(), color_mode, normalise_output, photoreceptors_local_adaptation_sensitivity, photoreceptors_temporal_constant, photoreceptors_spatial_constant, horizontal_cells_gain, hcells_temporal_constant, hcells_spatial_constant, ganglion_cells_sensitivity) }.into_result()
    }
    
    /// Set parameters values for the Inner Plexiform Layer (IPL) magnocellular channel
    ///
    /// this channel processes signals output from OPL processing stage in peripheral vision, it allows
    /// motion information enhancement. It is decorrelated from the details channel. See reference
    /// papers for more details.
    ///
    /// ## Parameters
    /// * normaliseOutput: specifies if (true) output is rescaled between 0 and 255 of not (false)
    /// * parasolCells_beta: the low pass filter gain used for local contrast adaptation at the
    /// IPL level of the retina (for ganglion cells local adaptation), typical value is 0
    /// * parasolCells_tau: the low pass filter time constant used for local contrast adaptation
    /// at the IPL level of the retina (for ganglion cells local adaptation), unit is frame, typical
    /// value is 0 (immediate response)
    /// * parasolCells_k: the low pass filter spatial constant used for local contrast adaptation
    /// at the IPL level of the retina (for ganglion cells local adaptation), unit is pixels, typical
    /// value is 5
    /// * amacrinCellsTemporalCutFrequency: the time constant of the first order high pass fiter of
    /// the magnocellular way (motion information channel), unit is frames, typical value is 1.2
    /// * V0CompressionParameter: the compression strengh of the ganglion cells local adaptation
    /// output, set a value between 0.6 and 1 for best results, a high value increases more the low
    /// value sensitivity... and the output saturates faster, recommended value: 0.95
    /// * localAdaptintegration_tau: specifies the temporal constant of the low pas filter
    /// involved in the computation of the local "motion mean" for the local adaptation computation
    /// * localAdaptintegration_k: specifies the spatial constant of the low pas filter involved
    /// in the computation of the local "motion mean" for the local adaptation computation
    ///
    /// ## C++ default parameters
    /// * normalise_output: true
    /// * parasol_cells_beta: 0.f
    /// * parasol_cells_tau: 0.f
    /// * parasol_cells_k: 7.f
    /// * amacrin_cells_temporal_cut_frequency: 1.2f
    /// * v0_compression_parameter: 0.95f
    /// * local_adaptintegration_tau: 0.f
    /// * local_adaptintegration_k: 7.f
    fn setup_ipl_magno_channel(&mut self, normalise_output: bool, parasol_cells_beta: f32, parasol_cells_tau: f32, parasol_cells_k: f32, amacrin_cells_temporal_cut_frequency: f32, v0_compression_parameter: f32, local_adaptintegration_tau: f32, local_adaptintegration_k: f32) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_setupIPLMagnoChannel_bool_float_float_float_float_float_float_float(self.as_raw_Retina(), normalise_output, parasol_cells_beta, parasol_cells_tau, parasol_cells_k, amacrin_cells_temporal_cut_frequency, v0_compression_parameter, local_adaptintegration_tau, local_adaptintegration_k) }.into_result()
    }
    
    /// Method which allows retina to be applied on an input image,
    ///
    /// after run, encapsulated retina module is ready to deliver its outputs using dedicated
    /// acccessors, see getParvo and getMagno methods
    /// ## Parameters
    /// * inputImage: the input Mat image to be processed, can be gray level or BGR coded in any
    /// format (from 8bit to 16bits)
    fn run(&mut self, input_image: &core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_run_Mat(self.as_raw_Retina(), input_image.as_raw_Mat()) }.into_result()
    }
    
    /// Method which processes an image in the aim to correct its luminance correct
    /// backlight problems, enhance details in shadows.
    ///
    /// This method is designed to perform High Dynamic Range image tone mapping (compress \>8bit/pixel
    /// images to 8bit/pixel). This is a simplified version of the Retina Parvocellular model
    /// (simplified version of the run/getParvo methods call) since it does not include the
    /// spatio-temporal filter modelling the Outer Plexiform Layer of the retina that performs spectral
    /// whitening and many other stuff. However, it works great for tone mapping and in a faster way.
    ///
    /// Check the demos and experiments section to see examples and the way to perform tone mapping
    /// using the original retina model and the method.
    ///
    /// ## Parameters
    /// * inputImage: the input image to process (should be coded in float format : CV_32F,
    /// CV_32FC1, CV_32F_C3, CV_32F_C4, the 4th channel won't be considered).
    /// * outputToneMappedImage: the output 8bit/channel tone mapped image (CV_8U or CV_8UC3 format).
    fn apply_fast_tone_mapping(&mut self, input_image: &core::Mat, output_tone_mapped_image: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_applyFastToneMapping_Mat_Mat(self.as_raw_Retina(), input_image.as_raw_Mat(), output_tone_mapped_image.as_raw_Mat()) }.into_result()
    }
    
    /// Accessor of the details channel of the retina (models foveal vision).
    ///
    /// Warning, getParvoRAW methods return buffers that are not rescaled within range [0;255] while
    /// the non RAW method allows a normalized matrix to be retrieved.
    ///
    /// ## Parameters
    /// * retinaOutput_parvo: the output buffer (reallocated if necessary), format can be :
    /// *   a Mat, this output is rescaled for standard 8bits image processing use in OpenCV
    /// *   RAW methods actually return a 1D matrix (encoding is R1, R2, ... Rn, G1, G2, ..., Gn, B1,
    /// B2, ...Bn), this output is the original retina filter model output, without any
    /// quantification or rescaling.
    /// @see getParvoRAW
    fn get_parvo(&mut self, retina_output_parvo: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_getParvo_Mat(self.as_raw_Retina(), retina_output_parvo.as_raw_Mat()) }.into_result()
    }
    
    /// Accessor of the details channel of the retina (models foveal vision).
    /// @see getParvo
    fn get_parvo_raw(&mut self, retina_output_parvo: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_getParvoRAW_Mat(self.as_raw_Retina(), retina_output_parvo.as_raw_Mat()) }.into_result()
    }
    
    /// Accessor of the motion channel of the retina (models peripheral vision).
    ///
    /// Warning, getMagnoRAW methods return buffers that are not rescaled within range [0;255] while
    /// the non RAW method allows a normalized matrix to be retrieved.
    /// ## Parameters
    /// * retinaOutput_magno: the output buffer (reallocated if necessary), format can be :
    /// *   a Mat, this output is rescaled for standard 8bits image processing use in OpenCV
    /// *   RAW methods actually return a 1D matrix (encoding is M1, M2,... Mn), this output is the
    /// original retina filter model output, without any quantification or rescaling.
    /// @see getMagnoRAW
    fn get_magno(&mut self, retina_output_magno: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_getMagno_Mat(self.as_raw_Retina(), retina_output_magno.as_raw_Mat()) }.into_result()
    }
    
    /// Accessor of the motion channel of the retina (models peripheral vision).
    /// @see getMagno
    fn get_magno_raw(&mut self, retina_output_magno: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_getMagnoRAW_Mat(self.as_raw_Retina(), retina_output_magno.as_raw_Mat()) }.into_result()
    }
    
    fn get_magno_raw_1(&self) -> Result<core::Mat> {
        unsafe { sys::cv_bioinspired_Retina_getMagnoRAW_const(self.as_raw_Retina()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_parvo_raw_1(&self) -> Result<core::Mat> {
        unsafe { sys::cv_bioinspired_Retina_getParvoRAW_const(self.as_raw_Retina()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Activate color saturation as the final step of the color demultiplexing process -\> this
    /// saturation is a sigmoide function applied to each channel of the demultiplexed image.
    /// ## Parameters
    /// * saturateColors: boolean that activates color saturation (if true) or desactivate (if false)
    /// * colorSaturationValue: the saturation factor : a simple factor applied on the chrominance
    /// buffers
    ///
    /// ## C++ default parameters
    /// * saturate_colors: true
    /// * color_saturation_value: 4.0f
    fn set_color_saturation(&mut self, saturate_colors: bool, color_saturation_value: f32) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_setColorSaturation_bool_float(self.as_raw_Retina(), saturate_colors, color_saturation_value) }.into_result()
    }
    
    /// Clears all retina buffers
    ///
    /// (equivalent to opening the eyes after a long period of eye close ;o) whatchout the temporal
    /// transition occuring just after this method call.
    fn clear_buffers(&mut self) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_clearBuffers(self.as_raw_Retina()) }.into_result()
    }
    
    /// Activate/desactivate the Magnocellular pathway processing (motion information extraction), by
    /// default, it is activated
    /// ## Parameters
    /// * activate: true if Magnocellular output should be activated, false if not... if activated,
    /// the Magnocellular output can be retrieved using the **getMagno** methods
    fn activate_moving_contours_processing(&mut self, activate: bool) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_activateMovingContoursProcessing_bool(self.as_raw_Retina(), activate) }.into_result()
    }
    
    /// Activate/desactivate the Parvocellular pathway processing (contours information extraction), by
    /// default, it is activated
    /// ## Parameters
    /// * activate: true if Parvocellular (contours information extraction) output should be
    /// activated, false if not... if activated, the Parvocellular output can be retrieved using the
    /// Retina::getParvo methods
    fn activate_contours_processing(&mut self, activate: bool) -> Result<()> {
        unsafe { sys::cv_bioinspired_Retina_activateContoursProcessing_bool(self.as_raw_Retina(), activate) }.into_result()
    }
    
}

impl dyn Retina + '_ {

    pub fn create(input_size: core::Size) -> Result<types::PtrOfRetina> {
        unsafe { sys::cv_bioinspired_Retina_create_Size(input_size) }.into_result().map(|ptr| types::PtrOfRetina { ptr })
    }
    
    /// Constructors from standardized interfaces : retreive a smart pointer to a Retina instance
    ///
    /// ## Parameters
    /// * inputSize: the input frame size
    /// * colorMode: the chosen processing mode : with or without color processing
    /// * colorSamplingMethod: specifies which kind of color sampling will be used :
    /// *   cv::bioinspired::RETINA_COLOR_RANDOM: each pixel position is either R, G or B in a random choice
    /// *   cv::bioinspired::RETINA_COLOR_DIAGONAL: color sampling is RGBRGBRGB..., line 2 BRGBRGBRG..., line 3, GBRGBRGBR...
    /// *   cv::bioinspired::RETINA_COLOR_BAYER: standard bayer sampling
    /// * useRetinaLogSampling: activate retina log sampling, if true, the 2 following parameters can
    /// be used
    /// * reductionFactor: only usefull if param useRetinaLogSampling=true, specifies the reduction
    /// factor of the output frame (as the center (fovea) is high resolution and corners can be
    /// underscaled, then a reduction of the output is allowed without precision leak
    /// * samplingStrenght: only usefull if param useRetinaLogSampling=true, specifies the strenght of
    /// the log scale that is applied
    ///
    /// ## C++ default parameters
    /// * color_sampling_method: RETINA_COLOR_BAYER
    /// * use_retina_log_sampling: false
    /// * reduction_factor: 1.0f
    /// * sampling_strenght: 10.0f
    pub fn create_1(input_size: core::Size, color_mode: bool, color_sampling_method: i32, use_retina_log_sampling: bool, reduction_factor: f32, sampling_strenght: f32) -> Result<types::PtrOfRetina> {
        unsafe { sys::cv_bioinspired_Retina_create_Size_bool_int_bool_float_float(input_size, color_mode, color_sampling_method, use_retina_log_sampling, reduction_factor, sampling_strenght) }.into_result().map(|ptr| types::PtrOfRetina { ptr })
    }
    
}

// Generating impl for trait cv::bioinspired::RetinaFastToneMapping (trait)
/// a wrapper class which allows the tone mapping algorithm of Meylan&al(2007) to be used with OpenCV.
///
/// This algorithm is already implemented in thre Retina class (retina::applyFastToneMapping) but used it does not require all the retina model to be allocated. This allows a light memory use for low memory devices (smartphones, etc.
/// As a summary, these are the model properties:
/// - 2 stages of local luminance adaptation with a different local neighborhood for each.
/// - first stage models the retina photorecetors local luminance adaptation
/// - second stage models th ganglion cells local information adaptation
/// - compared to the initial publication, this class uses spatio-temporal low pass filters instead of spatial only filters.
/// this can help noise robustness and temporal stability for video sequence use cases.
///
/// for more information, read to the following papers :
/// Meylan L., Alleysson D., and Susstrunk S., A Model of Retinal Local Adaptation for the Tone Mapping of Color Filter Array Images, Journal of Optical Society of America, A, Vol. 24, N 9, September, 1st, 2007, pp. 2807-2816Benoit A., Caplier A., Durette B., Herault, J., "USING HUMAN VISUAL SYSTEM MODELING FOR BIO-INSPIRED LOW LEVEL IMAGE PROCESSING", Elsevier, Computer Vision and Image Understanding 114 (2010), pp. 758-773, DOI: http://dx.doi.org/10.1016/j.cviu.2010.01.011
/// regarding spatio-temporal filter and the bigger retina model :
/// Vision: Images, Signals and Neural Networks: Models of Neural Processing in Visual Perception (Progress in Neural Processing),By: Jeanny Herault, ISBN: 9814273686. WAPI (Tower ID): 113266891.
pub trait RetinaFastToneMapping: core::Algorithm {
    #[inline(always)] fn as_raw_RetinaFastToneMapping(&self) -> *mut c_void;
    /// applies a luminance correction (initially High Dynamic Range (HDR) tone mapping)
    ///
    /// using only the 2 local adaptation stages of the retina parvocellular channel : photoreceptors
    /// level and ganlion cells level. Spatio temporal filtering is applied but limited to temporal
    /// smoothing and eventually high frequencies attenuation. This is a lighter method than the one
    /// available using the regular retina::run method. It is then faster but it does not include
    /// complete temporal filtering nor retina spectral whitening. Then, it can have a more limited
    /// effect on images with a very high dynamic range. This is an adptation of the original still
    /// image HDR tone mapping algorithm of David Alleyson, Sabine Susstruck and Laurence Meylan's
    /// work, please cite: -> Meylan L., Alleysson D., and Susstrunk S., A Model of Retinal Local
    /// Adaptation for the Tone Mapping of Color Filter Array Images, Journal of Optical Society of
    /// America, A, Vol. 24, N 9, September, 1st, 2007, pp. 2807-2816
    ///
    /// ## Parameters
    /// * inputImage: the input image to process RGB or gray levels
    /// * outputToneMappedImage: the output tone mapped image
    fn apply_fast_tone_mapping(&mut self, input_image: &core::Mat, output_tone_mapped_image: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_Mat_Mat(self.as_raw_RetinaFastToneMapping(), input_image.as_raw_Mat(), output_tone_mapped_image.as_raw_Mat()) }.into_result()
    }
    
    /// updates tone mapping behaviors by adjusing the local luminance computation area
    ///
    /// ## Parameters
    /// * photoreceptorsNeighborhoodRadius: the first stage local adaptation area
    /// * ganglioncellsNeighborhoodRadius: the second stage local adaptation area
    /// * meanLuminanceModulatorK: the factor applied to modulate the meanLuminance information
    /// (default is 1, see reference paper)
    ///
    /// ## C++ default parameters
    /// * photoreceptors_neighborhood_radius: 3.f
    /// * ganglioncells_neighborhood_radius: 1.f
    /// * mean_luminance_modulator_k: 1.f
    fn setup(&mut self, photoreceptors_neighborhood_radius: f32, ganglioncells_neighborhood_radius: f32, mean_luminance_modulator_k: f32) -> Result<()> {
        unsafe { sys::cv_bioinspired_RetinaFastToneMapping_setup_float_float_float(self.as_raw_RetinaFastToneMapping(), photoreceptors_neighborhood_radius, ganglioncells_neighborhood_radius, mean_luminance_modulator_k) }.into_result()
    }
    
}

impl dyn RetinaFastToneMapping + '_ {

    pub fn create(input_size: core::Size) -> Result<types::PtrOfRetinaFastToneMapping> {
        unsafe { sys::cv_bioinspired_RetinaFastToneMapping_create_Size(input_size) }.into_result().map(|ptr| types::PtrOfRetinaFastToneMapping { ptr })
    }
    
}

// boxed class cv::bioinspired::RetinaParameters
/// retina model parameters structure
///
/// For better clarity, check explenations on the comments of methods : setupOPLandIPLParvoChannel and setupIPLMagnoChannel
///
/// Here is the default configuration file of the retina module. It gives results such as the first
/// retina output shown on the top of this page.
///
/// ```ignore{xml}
/// <?xml version="1.0"?>
/// <opencv_storage>
/// <OPLandIPLparvo>
/// <colorMode>1</colorMode>
/// <normaliseOutput>1</normaliseOutput>
/// <photoreceptorsLocalAdaptationSensitivity>7.5e-01</photoreceptorsLocalAdaptationSensitivity>
/// <photoreceptorsTemporalConstant>9.0e-01</photoreceptorsTemporalConstant>
/// <photoreceptorsSpatialConstant>5.3e-01</photoreceptorsSpatialConstant>
/// <horizontalCellsGain>0.01</horizontalCellsGain>
/// <hcellsTemporalConstant>0.5</hcellsTemporalConstant>
/// <hcellsSpatialConstant>7.</hcellsSpatialConstant>
/// <ganglionCellsSensitivity>7.5e-01</ganglionCellsSensitivity></OPLandIPLparvo>
/// <IPLmagno>
/// <normaliseOutput>1</normaliseOutput>
/// <parasolCells_beta>0.</parasolCells_beta>
/// <parasolCells_tau>0.</parasolCells_tau>
/// <parasolCells_k>7.</parasolCells_k>
/// <amacrinCellsTemporalCutFrequency>2.0e+00</amacrinCellsTemporalCutFrequency>
/// <V0CompressionParameter>9.5e-01</V0CompressionParameter>
/// <localAdaptintegration_tau>0.</localAdaptintegration_tau>
/// <localAdaptintegration_k>7.</localAdaptintegration_k></IPLmagno>
/// </opencv_storage>
/// ```
///
///
/// Here is the 'realistic" setup used to obtain the second retina output shown on the top of this page.
///
/// ```ignore{xml}
/// <?xml version="1.0"?>
/// <opencv_storage>
/// <OPLandIPLparvo>
/// <colorMode>1</colorMode>
/// <normaliseOutput>1</normaliseOutput>
/// <photoreceptorsLocalAdaptationSensitivity>8.9e-01</photoreceptorsLocalAdaptationSensitivity>
/// <photoreceptorsTemporalConstant>9.0e-01</photoreceptorsTemporalConstant>
/// <photoreceptorsSpatialConstant>5.3e-01</photoreceptorsSpatialConstant>
/// <horizontalCellsGain>0.3</horizontalCellsGain>
/// <hcellsTemporalConstant>0.5</hcellsTemporalConstant>
/// <hcellsSpatialConstant>7.</hcellsSpatialConstant>
/// <ganglionCellsSensitivity>8.9e-01</ganglionCellsSensitivity></OPLandIPLparvo>
/// <IPLmagno>
/// <normaliseOutput>1</normaliseOutput>
/// <parasolCells_beta>0.</parasolCells_beta>
/// <parasolCells_tau>0.</parasolCells_tau>
/// <parasolCells_k>7.</parasolCells_k>
/// <amacrinCellsTemporalCutFrequency>2.0e+00</amacrinCellsTemporalCutFrequency>
/// <V0CompressionParameter>9.5e-01</V0CompressionParameter>
/// <localAdaptintegration_tau>0.</localAdaptintegration_tau>
/// <localAdaptintegration_k>7.</localAdaptintegration_k></IPLmagno>
/// </opencv_storage>
/// ```
pub struct RetinaParameters {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::bioinspired::RetinaParameters {
    fn drop(&mut self) {
        unsafe { sys::cv_RetinaParameters_delete(self.ptr) };
    }
}
impl crate::bioinspired::RetinaParameters {
    #[inline(always)] pub fn as_raw_RetinaParameters(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RetinaParameters {}

// boxed class cv::bioinspired::RetinaParameters::IplMagnoParameters
/// Inner Plexiform Layer Magnocellular channel (IplMagno)
pub struct RetinaParameters_IplMagnoParameters {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::bioinspired::RetinaParameters_IplMagnoParameters {
    fn drop(&mut self) {
        unsafe { sys::cv_RetinaParameters_IplMagnoParameters_delete(self.ptr) };
    }
}
impl crate::bioinspired::RetinaParameters_IplMagnoParameters {
    #[inline(always)] pub fn as_raw_RetinaParameters_IplMagnoParameters(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RetinaParameters_IplMagnoParameters {}

impl RetinaParameters_IplMagnoParameters {

    pub fn new() -> Result<crate::bioinspired::RetinaParameters_IplMagnoParameters> {
        unsafe { sys::cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters() }.into_result().map(|ptr| crate::bioinspired::RetinaParameters_IplMagnoParameters { ptr })
    }
    
}

// boxed class cv::bioinspired::RetinaParameters::OPLandIplParvoParameters
/// Outer Plexiform Layer (OPL) and Inner Plexiform Layer Parvocellular (IplParvo) parameters
pub struct RetinaParameters_OPLandIplParvoParameters {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::bioinspired::RetinaParameters_OPLandIplParvoParameters {
    fn drop(&mut self) {
        unsafe { sys::cv_RetinaParameters_OPLandIplParvoParameters_delete(self.ptr) };
    }
}
impl crate::bioinspired::RetinaParameters_OPLandIplParvoParameters {
    #[inline(always)] pub fn as_raw_RetinaParameters_OPLandIplParvoParameters(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for RetinaParameters_OPLandIplParvoParameters {}

impl RetinaParameters_OPLandIplParvoParameters {

    pub fn new() -> Result<crate::bioinspired::RetinaParameters_OPLandIplParvoParameters> {
        unsafe { sys::cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters() }.into_result().map(|ptr| crate::bioinspired::RetinaParameters_OPLandIplParvoParameters { ptr })
    }
    
}

// boxed class cv::bioinspired::SegmentationParameters
/// parameter structure that stores the transient events detector setup parameters
pub struct SegmentationParameters {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::bioinspired::SegmentationParameters {
    fn drop(&mut self) {
        unsafe { sys::cv_SegmentationParameters_delete(self.ptr) };
    }
}
impl crate::bioinspired::SegmentationParameters {
    #[inline(always)] pub fn as_raw_SegmentationParameters(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SegmentationParameters {}

impl SegmentationParameters {

    pub fn new() -> Result<crate::bioinspired::SegmentationParameters> {
        unsafe { sys::cv_bioinspired_SegmentationParameters_SegmentationParameters() }.into_result().map(|ptr| crate::bioinspired::SegmentationParameters { ptr })
    }
    
}

// Generating impl for trait cv::bioinspired::TransientAreasSegmentationModule (trait)
/// class which provides a transient/moving areas segmentation module
///
/// perform a locally adapted segmentation by using the retina magno input data Based on Alexandre
/// BENOIT thesis: "Le système visuel humain au secours de la vision par ordinateur"
///
/// 3 spatio temporal filters are used:
/// - a first one which filters the noise and local variations of the input motion energy
/// - a second (more powerfull low pass spatial filter) which gives the neighborhood motion energy the
/// segmentation consists in the comparison of these both outputs, if the local motion energy is higher
/// to the neighborhood otion energy, then the area is considered as moving and is segmented
/// - a stronger third low pass filter helps decision by providing a smooth information about the
/// "motion context" in a wider area
pub trait TransientAreasSegmentationModule: core::Algorithm {
    #[inline(always)] fn as_raw_TransientAreasSegmentationModule(&self) -> *mut c_void;
    /// return the sze of the manage input and output images
    fn get_size(&mut self) -> Result<core::Size> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getSize(self.as_raw_TransientAreasSegmentationModule()) }.into_result()
    }
    
    /// try to open an XML segmentation parameters file to adjust current segmentation instance setup
    ///
    /// - if the xml file does not exist, then default setup is applied
    /// - warning, Exceptions are thrown if read XML file is not valid
    /// ## Parameters
    /// * segmentationParameterFile: : the parameters filename
    /// * applyDefaultSetupOnFailure: : set to true if an error must be thrown on error
    ///
    /// ## C++ default parameters
    /// * segmentation_parameter_file: ""
    /// * apply_default_setup_on_failure: true
    fn setup(&mut self, segmentation_parameter_file: &str, apply_default_setup_on_failure: bool) -> Result<()> {
        string_arg!(mut segmentation_parameter_file);
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_String_bool(self.as_raw_TransientAreasSegmentationModule(), segmentation_parameter_file.as_ptr() as _, apply_default_setup_on_failure) }.into_result()
    }
    
    /// try to open an XML segmentation parameters file to adjust current segmentation instance setup
    ///
    /// - if the xml file does not exist, then default setup is applied
    /// - warning, Exceptions are thrown if read XML file is not valid
    /// ## Parameters
    /// * newParameters: : a parameters structures updated with the new target configuration
    fn setup_1(&mut self, new_parameters: &crate::bioinspired::SegmentationParameters) -> Result<()> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(self.as_raw_TransientAreasSegmentationModule(), new_parameters.as_raw_SegmentationParameters()) }.into_result()
    }
    
    /// return the current parameters setup
    fn get_parameters(&mut self) -> Result<crate::bioinspired::SegmentationParameters> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getParameters(self.as_raw_TransientAreasSegmentationModule()) }.into_result().map(|ptr| crate::bioinspired::SegmentationParameters { ptr })
    }
    
    /// parameters setup display method
    /// ## Returns
    /// a string which contains formatted parameters information
    fn print_setup(&mut self) -> Result<String> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_printSetup(self.as_raw_TransientAreasSegmentationModule()) }.into_result().map(crate::templ::receive_string)
    }
    
    /// write xml/yml formated parameters information
    /// ## Parameters
    /// * fs: : the filename of the xml file that will be open and writen with formatted parameters information
    fn write(&self, fs: &str) -> Result<()> {
        string_arg!(mut fs);
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_write_const_String(self.as_raw_TransientAreasSegmentationModule(), fs.as_ptr() as _) }.into_result()
    }
    
    /// main processing method, get result using methods getSegmentationPicture()
    /// ## Parameters
    /// * inputToSegment: : the image to process, it must match the instance buffer size !
    /// * channelIndex: : the channel to process in case of multichannel images
    ///
    /// ## C++ default parameters
    /// * channel_index: 0
    fn run(&mut self, input_to_segment: &core::Mat, channel_index: i32) -> Result<()> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_run_Mat_int(self.as_raw_TransientAreasSegmentationModule(), input_to_segment.as_raw_Mat(), channel_index) }.into_result()
    }
    
    /// access function
    /// return the last segmentation result: a boolean picture which is resampled between 0 and 255 for a display purpose
    fn get_segmentation_picture(&mut self, transient_areas: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_Mat(self.as_raw_TransientAreasSegmentationModule(), transient_areas.as_raw_Mat()) }.into_result()
    }
    
    /// cleans all the buffers of the instance
    fn clear_all_buffers(&mut self) -> Result<()> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(self.as_raw_TransientAreasSegmentationModule()) }.into_result()
    }
    
}

impl dyn TransientAreasSegmentationModule + '_ {

    /// allocator
    /// ## Parameters
    /// * inputSize: : size of the images input to segment (output will be the same size)
    pub fn create(input_size: core::Size) -> Result<types::PtrOfTransientAreasSegmentationModule> {
        unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_create_Size(input_size) }.into_result().map(|ptr| types::PtrOfTransientAreasSegmentationModule { ptr })
    }
    
}


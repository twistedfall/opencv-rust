pub mod bioinspired {
	//! # Biologically inspired vision models and derivated tools
	//! 
	//! The module provides biological visual systems models (human visual system and others). It also
	//! provides derivated objects that take advantage of those bio-inspired models.
	//! 
	//! [bioinspired_retina]
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::RetinaParametersTraitConst, super::RetinaParametersTrait, super::RetinaTraitConst, super::RetinaTrait, super::RetinaFastToneMappingTraitConst, super::RetinaFastToneMappingTrait, super::TransientAreasSegmentationModuleTraitConst, super::TransientAreasSegmentationModuleTrait };
	}
	
	/// standard bayer sampling
	pub const RETINA_COLOR_BAYER: i32 = 2;
	/// color sampling is RGBRGBRGB..., line 2 BRGBRGBRG..., line 3, GBRGBRGBR...
	pub const RETINA_COLOR_DIAGONAL: i32 = 1;
	/// each pixel position is either R, G or B in a random choice
	pub const RETINA_COLOR_RANDOM: i32 = 0;
	/// Constant methods for [crate::bioinspired::Retina]
	pub trait RetinaTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Retina(&self) -> *const c_void;
	
		/// Write xml/yml formated parameters information
		/// ## Parameters
		/// * fs: the filename of the xml file that will be open and writen with formatted parameters
		/// information
		#[inline]
		fn write(&self, fs: &str) -> Result<()> {
			extern_container_arg!(mut fs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_write_const_String(self.as_raw_Retina(), fs.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Write xml/yml formated parameters information
		/// ## Parameters
		/// * fs: the filename of the xml file that will be open and writen with formatted parameters
		/// information
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn write_to_storage(&self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_write_const_FileStorageR(self.as_raw_Retina(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Accessor of the motion channel of the retina (models peripheral vision).
		/// ## See also
		/// getMagno
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn get_magno_raw(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getMagnoRAW_const(self.as_raw_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Accessor of the details channel of the retina (models foveal vision).
		/// ## See also
		/// getParvo
		/// 
		/// ## Overloaded parameters
		#[inline]
		fn get_parvo_raw(&self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getParvoRAW_const(self.as_raw_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::bioinspired::Retina]
	pub trait RetinaTrait: core::AlgorithmTrait + crate::bioinspired::RetinaTraitConst {
		fn as_raw_mut_Retina(&mut self) -> *mut c_void;
	
		/// Retreive retina input buffer size
		/// ## Returns
		/// the retina input buffer size
		#[inline]
		fn get_input_size(&mut self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getInputSize(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Retreive retina output buffer size that can be different from the input if a spatial log
		/// transformation is applied
		/// ## Returns
		/// the retina output buffer size
		#[inline]
		fn get_output_size(&mut self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getOutputSize(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn setup_from_file(&mut self, retina_parameter_file: &str, apply_default_setup_on_failure: bool) -> Result<()> {
			extern_container_arg!(mut retina_parameter_file);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setup_String_const_bool(self.as_raw_mut_Retina(), retina_parameter_file.opencv_as_extern_mut(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [setup_from_file] function uses the following default values for its arguments:
		/// * retina_parameter_file: ""
		/// * apply_default_setup_on_failure: true
		#[inline]
		fn setup_from_file_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setup(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Overloaded parameters
		/// 
		/// * fs: the open Filestorage which contains retina parameters
		/// * applyDefaultSetupOnFailure: set to true if an error must be thrown on error
		/// 
		/// ## C++ default parameters
		/// * apply_default_setup_on_failure: true
		#[inline]
		fn setup_from_storage(&mut self, fs: &mut core::FileStorage, apply_default_setup_on_failure: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setup_FileStorageR_const_bool(self.as_raw_mut_Retina(), fs.as_raw_mut_FileStorage(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @overload
		/// ## Parameters
		/// * fs: the open Filestorage which contains retina parameters
		/// * applyDefaultSetupOnFailure: set to true if an error must be thrown on error
		/// 
		/// ## Note
		/// This alternative version of [setup_from_storage] function uses the following default values for its arguments:
		/// * apply_default_setup_on_failure: true
		#[inline]
		fn setup_from_storage_def(&mut self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setup_FileStorageR(self.as_raw_mut_Retina(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Overloaded parameters
		/// 
		/// * newParameters: a parameters structures updated with the new target configuration.
		#[inline]
		fn setup(&mut self, mut new_parameters: crate::bioinspired::RetinaParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setup_RetinaParameters(self.as_raw_mut_Retina(), new_parameters.as_raw_mut_RetinaParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Returns
		/// the current parameters setup
		#[inline]
		fn get_parameters(&mut self) -> Result<crate::bioinspired::RetinaParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getParameters(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::bioinspired::RetinaParameters::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Outputs a string showing the used parameters setup
		/// ## Returns
		/// a string which contains formated parameters information
		#[inline]
		fn print_setup(&mut self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_printSetup(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Setup the OPL and IPL parvo channels (see biologocal model)
		/// 
		/// OPL is referred as Outer Plexiform Layer of the retina, it allows the spatio-temporal filtering
		/// which withens the spectrum and reduces spatio-temporal noise while attenuating global luminance
		/// (low frequency energy) IPL parvo is the OPL next processing stage, it refers to a part of the
		/// Inner Plexiform layer of the retina, it allows high contours sensitivity in foveal vision. See
		/// reference papers for more informations.
		/// for more informations, please have a look at the paper Benoit A., Caplier A., Durette B., Herault, J., "USING HUMAN VISUAL SYSTEM MODELING FOR BIO-INSPIRED LOW LEVEL IMAGE PROCESSING", Elsevier, Computer Vision and Image Understanding 114 (2010), pp. 758-773, DOI: <http://dx.doi.org/10.1016/j.cviu.2010.01.011>
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
		#[inline]
		fn setup_op_land_ipl_parvo_channel(&mut self, color_mode: bool, normalise_output: bool, photoreceptors_local_adaptation_sensitivity: f32, photoreceptors_temporal_constant: f32, photoreceptors_spatial_constant: f32, horizontal_cells_gain: f32, hcells_temporal_constant: f32, hcells_spatial_constant: f32, ganglion_cells_sensitivity: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setupOPLandIPLParvoChannel_const_bool_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(self.as_raw_mut_Retina(), color_mode, normalise_output, photoreceptors_local_adaptation_sensitivity, photoreceptors_temporal_constant, photoreceptors_spatial_constant, horizontal_cells_gain, hcells_temporal_constant, hcells_spatial_constant, ganglion_cells_sensitivity, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Setup the OPL and IPL parvo channels (see biologocal model)
		/// 
		/// OPL is referred as Outer Plexiform Layer of the retina, it allows the spatio-temporal filtering
		/// which withens the spectrum and reduces spatio-temporal noise while attenuating global luminance
		/// (low frequency energy) IPL parvo is the OPL next processing stage, it refers to a part of the
		/// Inner Plexiform layer of the retina, it allows high contours sensitivity in foveal vision. See
		/// reference papers for more informations.
		/// for more informations, please have a look at the paper Benoit A., Caplier A., Durette B., Herault, J., "USING HUMAN VISUAL SYSTEM MODELING FOR BIO-INSPIRED LOW LEVEL IMAGE PROCESSING", Elsevier, Computer Vision and Image Understanding 114 (2010), pp. 758-773, DOI: <http://dx.doi.org/10.1016/j.cviu.2010.01.011>
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
		/// ## Note
		/// This alternative version of [setup_op_land_ipl_parvo_channel] function uses the following default values for its arguments:
		/// * color_mode: true
		/// * normalise_output: true
		/// * photoreceptors_local_adaptation_sensitivity: 0.7f
		/// * photoreceptors_temporal_constant: 0.5f
		/// * photoreceptors_spatial_constant: 0.53f
		/// * horizontal_cells_gain: 0.f
		/// * hcells_temporal_constant: 1.f
		/// * hcells_spatial_constant: 7.f
		/// * ganglion_cells_sensitivity: 0.7f
		#[inline]
		fn setup_op_land_ipl_parvo_channel_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setupOPLandIPLParvoChannel(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn setup_ipl_magno_channel(&mut self, normalise_output: bool, parasol_cells_beta: f32, parasol_cells_tau: f32, parasol_cells_k: f32, amacrin_cells_temporal_cut_frequency: f32, v0_compression_parameter: f32, local_adaptintegration_tau: f32, local_adaptintegration_k: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setupIPLMagnoChannel_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(self.as_raw_mut_Retina(), normalise_output, parasol_cells_beta, parasol_cells_tau, parasol_cells_k, amacrin_cells_temporal_cut_frequency, v0_compression_parameter, local_adaptintegration_tau, local_adaptintegration_k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [setup_ipl_magno_channel] function uses the following default values for its arguments:
		/// * normalise_output: true
		/// * parasol_cells_beta: 0.f
		/// * parasol_cells_tau: 0.f
		/// * parasol_cells_k: 7.f
		/// * amacrin_cells_temporal_cut_frequency: 1.2f
		/// * v0_compression_parameter: 0.95f
		/// * local_adaptintegration_tau: 0.f
		/// * local_adaptintegration_k: 7.f
		#[inline]
		fn setup_ipl_magno_channel_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setupIPLMagnoChannel(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Method which allows retina to be applied on an input image,
		/// 
		/// after run, encapsulated retina module is ready to deliver its outputs using dedicated
		/// acccessors, see getParvo and getMagno methods
		/// ## Parameters
		/// * inputImage: the input Mat image to be processed, can be gray level or BGR coded in any
		/// format (from 8bit to 16bits)
		#[inline]
		fn run(&mut self, input_image: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(input_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_run_const__InputArrayR(self.as_raw_mut_Retina(), input_image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn apply_fast_tone_mapping(&mut self, input_image: &impl core::ToInputArray, output_tone_mapped_image: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(input_image);
			output_array_arg!(output_tone_mapped_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Retina(), input_image.as_raw__InputArray(), output_tone_mapped_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## See also
		/// getParvoRAW
		#[inline]
		fn get_parvo(&mut self, retina_output_parvo: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(retina_output_parvo);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getParvo_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_parvo.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Accessor of the details channel of the retina (models foveal vision).
		/// ## See also
		/// getParvo
		#[inline]
		fn get_parvo_raw_to(&mut self, retina_output_parvo: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(retina_output_parvo);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getParvoRAW_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_parvo.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## See also
		/// getMagnoRAW
		#[inline]
		fn get_magno(&mut self, retina_output_magno: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(retina_output_magno);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getMagno_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_magno.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Accessor of the motion channel of the retina (models peripheral vision).
		/// ## See also
		/// getMagno
		#[inline]
		fn get_magno_raw_to(&mut self, retina_output_magno: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(retina_output_magno);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR(self.as_raw_mut_Retina(), retina_output_magno.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn set_color_saturation(&mut self, saturate_colors: bool, color_saturation_value: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setColorSaturation_const_bool_const_float(self.as_raw_mut_Retina(), saturate_colors, color_saturation_value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Activate color saturation as the final step of the color demultiplexing process -\> this
		/// saturation is a sigmoide function applied to each channel of the demultiplexed image.
		/// ## Parameters
		/// * saturateColors: boolean that activates color saturation (if true) or desactivate (if false)
		/// * colorSaturationValue: the saturation factor : a simple factor applied on the chrominance
		/// buffers
		/// 
		/// ## Note
		/// This alternative version of [set_color_saturation] function uses the following default values for its arguments:
		/// * saturate_colors: true
		/// * color_saturation_value: 4.0f
		#[inline]
		fn set_color_saturation_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_setColorSaturation(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Clears all retina buffers
		/// 
		/// (equivalent to opening the eyes after a long period of eye close ;o) whatchout the temporal
		/// transition occuring just after this method call.
		#[inline]
		fn clear_buffers(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_clearBuffers(self.as_raw_mut_Retina(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Activate/desactivate the Magnocellular pathway processing (motion information extraction), by
		/// default, it is activated
		/// ## Parameters
		/// * activate: true if Magnocellular output should be activated, false if not... if activated,
		/// the Magnocellular output can be retrieved using the **getMagno** methods
		#[inline]
		fn activate_moving_contours_processing(&mut self, activate: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_activateMovingContoursProcessing_const_bool(self.as_raw_mut_Retina(), activate, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Activate/desactivate the Parvocellular pathway processing (contours information extraction), by
		/// default, it is activated
		/// ## Parameters
		/// * activate: true if Parvocellular (contours information extraction) output should be
		/// activated, false if not... if activated, the Parvocellular output can be retrieved using the
		/// Retina::getParvo methods
		#[inline]
		fn activate_contours_processing(&mut self, activate: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_activateContoursProcessing_const_bool(self.as_raw_mut_Retina(), activate, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
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
	///      _using the getParvo method output matrix : texture analysiswith enhanced signal to noise ratio and enhanced details robust against input images luminance ranges
	///      _using the getMagno method output matrix : motion analysis also with the previously cited properties
	/// 
	/// for more information, reer to the following papers :
	/// Benoit A., Caplier A., Durette B., Herault, J., "USING HUMAN VISUAL SYSTEM MODELING FOR BIO-INSPIRED LOW LEVEL IMAGE PROCESSING", Elsevier, Computer Vision and Image Understanding 114 (2010), pp. 758-773, DOI: <http://dx.doi.org/10.1016/j.cviu.2010.01.011>
	/// Vision: Images, Signals and Neural Networks: Models of Neural Processing in Visual Perception (Progress in Neural Processing),By: Jeanny Herault, ISBN: 9814273686. WAPI (Tower ID): 113266891.
	/// 
	/// The retina filter includes the research contributions of phd/research collegues from which code has been redrawn by the author :
	/// take a look at the retinacolor.hpp module to discover Brice Chaix de Lavarene color mosaicing/demosaicing and the reference paper:
	/// B. Chaix de Lavarene, D. Alleysson, B. Durette, J. Herault (2007). "Efficient demosaicing through recursive filtering", IEEE International Conference on Image Processing ICIP 2007
	/// take a look at imagelogpolprojection.hpp to discover retina spatial log sampling which originates from Barthelemy Durette phd with Jeanny Herault. A Retina / V1 cortex projection is also proposed and originates from Jeanny's discussions.
	/// more informations in the above cited Jeanny Heraults's book.
	pub struct Retina {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Retina }
	
	impl Drop for Retina {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_bioinspired_Retina_delete(self.as_raw_mut_Retina()) };
		}
	}
	
	unsafe impl Send for Retina {}
	
	impl core::AlgorithmTraitConst for Retina {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Retina {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaTraitConst for Retina {
		#[inline] fn as_raw_Retina(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::bioinspired::RetinaTrait for Retina {
		#[inline] fn as_raw_mut_Retina(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Retina {
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
		/// * samplingStrength: only usefull if param useRetinaLogSampling=true, specifies the strength of
		/// the log scale that is applied
		/// 
		/// ## Overloaded parameters
		#[inline]
		pub fn create(input_size: core::Size) -> Result<core::Ptr<crate::bioinspired::Retina>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_create_Size(input_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::bioinspired::Retina>::opencv_from_extern(ret) };
			Ok(ret)
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
		/// * samplingStrength: only usefull if param useRetinaLogSampling=true, specifies the strength of
		/// the log scale that is applied
		/// 
		/// ## C++ default parameters
		/// * color_sampling_method: RETINA_COLOR_BAYER
		/// * use_retina_log_sampling: false
		/// * reduction_factor: 1.0f
		/// * sampling_strength: 10.0f
		#[inline]
		pub fn create_ext(input_size: core::Size, color_mode: bool, color_sampling_method: i32, use_retina_log_sampling: bool, reduction_factor: f32, sampling_strength: f32) -> Result<core::Ptr<crate::bioinspired::Retina>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_create_Size_const_bool_int_const_bool_const_float_const_float(input_size.opencv_as_extern(), color_mode, color_sampling_method, use_retina_log_sampling, reduction_factor, sampling_strength, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::bioinspired::Retina>::opencv_from_extern(ret) };
			Ok(ret)
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
		/// * samplingStrength: only usefull if param useRetinaLogSampling=true, specifies the strength of
		/// the log scale that is applied
		/// 
		/// ## Note
		/// This alternative version of [create_ext] function uses the following default values for its arguments:
		/// * color_sampling_method: RETINA_COLOR_BAYER
		/// * use_retina_log_sampling: false
		/// * reduction_factor: 1.0f
		/// * sampling_strength: 10.0f
		#[inline]
		pub fn create_ext_def(input_size: core::Size, color_mode: bool) -> Result<core::Ptr<crate::bioinspired::Retina>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_Retina_create_Size_const_bool(input_size.opencv_as_extern(), color_mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::bioinspired::Retina>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Retina, core::Algorithm, cv_bioinspired_Retina_to_Algorithm }
	
	impl std::fmt::Debug for Retina {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Retina")
				.finish()
		}
	}
	
	/// Constant methods for [crate::bioinspired::RetinaFastToneMapping]
	pub trait RetinaFastToneMappingTraitConst: core::AlgorithmTraitConst {
		fn as_raw_RetinaFastToneMapping(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::bioinspired::RetinaFastToneMapping]
	pub trait RetinaFastToneMappingTrait: core::AlgorithmTrait + crate::bioinspired::RetinaFastToneMappingTraitConst {
		fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void;
	
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
		#[inline]
		fn apply_fast_tone_mapping(&mut self, input_image: &impl core::ToInputArray, output_tone_mapped_image: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(input_image);
			output_array_arg!(output_tone_mapped_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_RetinaFastToneMapping(), input_image.as_raw__InputArray(), output_tone_mapped_image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn setup(&mut self, photoreceptors_neighborhood_radius: f32, ganglioncells_neighborhood_radius: f32, mean_luminance_modulator_k: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaFastToneMapping_setup_const_float_const_float_const_float(self.as_raw_mut_RetinaFastToneMapping(), photoreceptors_neighborhood_radius, ganglioncells_neighborhood_radius, mean_luminance_modulator_k, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// updates tone mapping behaviors by adjusing the local luminance computation area
		/// 
		/// ## Parameters
		/// * photoreceptorsNeighborhoodRadius: the first stage local adaptation area
		/// * ganglioncellsNeighborhoodRadius: the second stage local adaptation area
		/// * meanLuminanceModulatorK: the factor applied to modulate the meanLuminance information
		/// (default is 1, see reference paper)
		/// 
		/// ## Note
		/// This alternative version of [setup] function uses the following default values for its arguments:
		/// * photoreceptors_neighborhood_radius: 3.f
		/// * ganglioncells_neighborhood_radius: 1.f
		/// * mean_luminance_modulator_k: 1.f
		#[inline]
		fn setup_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaFastToneMapping_setup(self.as_raw_mut_RetinaFastToneMapping(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// a wrapper class which allows the tone mapping algorithm of Meylan&al(2007) to be used with OpenCV.
	/// 
	/// This algorithm is already implemented in thre Retina class (retina::applyFastToneMapping) but used it does not require all the retina model to be allocated. This allows a light memory use for low memory devices (smartphones, etc.
	/// As a summary, these are the model properties:
	/// - 2 stages of local luminance adaptation with a different local neighborhood for each.
	/// - first stage models the retina photorecetors local luminance adaptation
	/// - second stage models th ganglion cells local information adaptation
	/// - compared to the initial publication, this class uses spatio-temporal low pass filters instead of spatial only filters.
	///   this can help noise robustness and temporal stability for video sequence use cases.
	/// 
	/// for more information, read to the following papers :
	/// Meylan L., Alleysson D., and Susstrunk S., A Model of Retinal Local Adaptation for the Tone Mapping of Color Filter Array Images, Journal of Optical Society of America, A, Vol. 24, N 9, September, 1st, 2007, pp. 2807-2816Benoit A., Caplier A., Durette B., Herault, J., "USING HUMAN VISUAL SYSTEM MODELING FOR BIO-INSPIRED LOW LEVEL IMAGE PROCESSING", Elsevier, Computer Vision and Image Understanding 114 (2010), pp. 758-773, DOI: <http://dx.doi.org/10.1016/j.cviu.2010.01.011>
	/// regarding spatio-temporal filter and the bigger retina model :
	/// Vision: Images, Signals and Neural Networks: Models of Neural Processing in Visual Perception (Progress in Neural Processing),By: Jeanny Herault, ISBN: 9814273686. WAPI (Tower ID): 113266891.
	pub struct RetinaFastToneMapping {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RetinaFastToneMapping }
	
	impl Drop for RetinaFastToneMapping {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_bioinspired_RetinaFastToneMapping_delete(self.as_raw_mut_RetinaFastToneMapping()) };
		}
	}
	
	unsafe impl Send for RetinaFastToneMapping {}
	
	impl core::AlgorithmTraitConst for RetinaFastToneMapping {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for RetinaFastToneMapping {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaFastToneMappingTraitConst for RetinaFastToneMapping {
		#[inline] fn as_raw_RetinaFastToneMapping(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::bioinspired::RetinaFastToneMappingTrait for RetinaFastToneMapping {
		#[inline] fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RetinaFastToneMapping {
		#[inline]
		pub fn create(input_size: core::Size) -> Result<core::Ptr<crate::bioinspired::RetinaFastToneMapping>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaFastToneMapping_create_Size(input_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::bioinspired::RetinaFastToneMapping>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { RetinaFastToneMapping, core::Algorithm, cv_bioinspired_RetinaFastToneMapping_to_Algorithm }
	
	impl std::fmt::Debug for RetinaFastToneMapping {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RetinaFastToneMapping")
				.finish()
		}
	}
	
	/// Constant methods for [crate::bioinspired::RetinaParameters]
	pub trait RetinaParametersTraitConst {
		fn as_raw_RetinaParameters(&self) -> *const c_void;
	
		#[inline]
		fn op_land_ipl_parvo(&self) -> crate::bioinspired::RetinaParameters_OPLandIplParvoParameters {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaParameters_propOPLandIplParvo_const(self.as_raw_RetinaParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn ipl_magno(&self) -> crate::bioinspired::RetinaParameters_IplMagnoParameters {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaParameters_propIplMagno_const(self.as_raw_RetinaParameters(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
	}
	
	/// Mutable methods for [crate::bioinspired::RetinaParameters]
	pub trait RetinaParametersTrait: crate::bioinspired::RetinaParametersTraitConst {
		fn as_raw_mut_RetinaParameters(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_op_land_ipl_parvo(&mut self, val: crate::bioinspired::RetinaParameters_OPLandIplParvoParameters) {
			let ret = unsafe { sys::cv_bioinspired_RetinaParameters_propOPLandIplParvo_OPLandIplParvoParameters(self.as_raw_mut_RetinaParameters(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_ipl_magno(&mut self, val: crate::bioinspired::RetinaParameters_IplMagnoParameters) {
			let ret = unsafe { sys::cv_bioinspired_RetinaParameters_propIplMagno_IplMagnoParameters(self.as_raw_mut_RetinaParameters(), val.opencv_as_extern()) };
			ret
		}
		
	}
	
	/// retina model parameters structure
	/// 
	/// For better clarity, check explenations on the comments of methods : setupOPLandIPLParvoChannel and setupIPLMagnoChannel
	/// 
	/// Here is the default configuration file of the retina module. It gives results such as the first
	/// retina output shown on the top of this page.
	/// 
	/// ```C++
	/// <?xml version="1.0"?>
	/// <opencv_storage>
	/// <OPLandIPLparvo>
	///    <colorMode>1</colorMode>
	///    <normaliseOutput>1</normaliseOutput>
	///    <photoreceptorsLocalAdaptationSensitivity>7.5e-01</photoreceptorsLocalAdaptationSensitivity>
	///    <photoreceptorsTemporalConstant>9.0e-01</photoreceptorsTemporalConstant>
	///    <photoreceptorsSpatialConstant>5.3e-01</photoreceptorsSpatialConstant>
	///    <horizontalCellsGain>0.01</horizontalCellsGain>
	///    <hcellsTemporalConstant>0.5</hcellsTemporalConstant>
	///    <hcellsSpatialConstant>7.</hcellsSpatialConstant>
	///    <ganglionCellsSensitivity>7.5e-01</ganglionCellsSensitivity></OPLandIPLparvo>
	/// <IPLmagno>
	///    <normaliseOutput>1</normaliseOutput>
	///    <parasolCells_beta>0.</parasolCells_beta>
	///    <parasolCells_tau>0.</parasolCells_tau>
	///    <parasolCells_k>7.</parasolCells_k>
	///    <amacrinCellsTemporalCutFrequency>2.0e+00</amacrinCellsTemporalCutFrequency>
	///    <V0CompressionParameter>9.5e-01</V0CompressionParameter>
	///    <localAdaptintegration_tau>0.</localAdaptintegration_tau>
	///    <localAdaptintegration_k>7.</localAdaptintegration_k></IPLmagno>
	/// </opencv_storage>
	/// ```
	/// 
	/// 
	/// Here is the 'realistic" setup used to obtain the second retina output shown on the top of this page.
	/// 
	/// ```C++
	/// <?xml version="1.0"?>
	/// <opencv_storage>
	/// <OPLandIPLparvo>
	///   <colorMode>1</colorMode>
	///   <normaliseOutput>1</normaliseOutput>
	///   <photoreceptorsLocalAdaptationSensitivity>8.9e-01</photoreceptorsLocalAdaptationSensitivity>
	///   <photoreceptorsTemporalConstant>9.0e-01</photoreceptorsTemporalConstant>
	///   <photoreceptorsSpatialConstant>5.3e-01</photoreceptorsSpatialConstant>
	///   <horizontalCellsGain>0.3</horizontalCellsGain>
	///   <hcellsTemporalConstant>0.5</hcellsTemporalConstant>
	///   <hcellsSpatialConstant>7.</hcellsSpatialConstant>
	///   <ganglionCellsSensitivity>8.9e-01</ganglionCellsSensitivity></OPLandIPLparvo>
	/// <IPLmagno>
	///   <normaliseOutput>1</normaliseOutput>
	///   <parasolCells_beta>0.</parasolCells_beta>
	///   <parasolCells_tau>0.</parasolCells_tau>
	///   <parasolCells_k>7.</parasolCells_k>
	///   <amacrinCellsTemporalCutFrequency>2.0e+00</amacrinCellsTemporalCutFrequency>
	///   <V0CompressionParameter>9.5e-01</V0CompressionParameter>
	///   <localAdaptintegration_tau>0.</localAdaptintegration_tau>
	///   <localAdaptintegration_k>7.</localAdaptintegration_k></IPLmagno>
	/// </opencv_storage>
	/// ```
	/// 
	pub struct RetinaParameters {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RetinaParameters }
	
	impl Drop for RetinaParameters {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_bioinspired_RetinaParameters_delete(self.as_raw_mut_RetinaParameters()) };
		}
	}
	
	unsafe impl Send for RetinaParameters {}
	
	impl crate::bioinspired::RetinaParametersTraitConst for RetinaParameters {
		#[inline] fn as_raw_RetinaParameters(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::bioinspired::RetinaParametersTrait for RetinaParameters {
		#[inline] fn as_raw_mut_RetinaParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RetinaParameters {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_bioinspired_RetinaParameters_defaultNew_const()) }
		}
		
	}
	
	impl Clone for RetinaParameters {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_bioinspired_RetinaParameters_implicitClone_const(self.as_raw_RetinaParameters())) }
		}
	}
	
	impl std::fmt::Debug for RetinaParameters {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RetinaParameters")
				.field("op_land_ipl_parvo", &crate::bioinspired::RetinaParametersTraitConst::op_land_ipl_parvo(self))
				.field("ipl_magno", &crate::bioinspired::RetinaParametersTraitConst::ipl_magno(self))
				.finish()
		}
	}
	
	impl Default for RetinaParameters {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Inner Plexiform Layer Magnocellular channel (IplMagno)
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct RetinaParameters_IplMagnoParameters {
		pub normalise_output: bool,
		pub parasol_cells_beta: f32,
		pub parasol_cells_tau: f32,
		pub parasol_cells_k: f32,
		pub amacrin_cells_temporal_cut_frequency: f32,
		pub v0_compression_parameter: f32,
		pub local_adaptintegration_tau: f32,
		pub local_adaptintegration_k: f32,
	}
	
	opencv_type_simple! { crate::bioinspired::RetinaParameters_IplMagnoParameters }
	
	impl RetinaParameters_IplMagnoParameters {
		#[inline]
		pub fn default() -> Result<crate::bioinspired::RetinaParameters_IplMagnoParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Outer Plexiform Layer (OPL) and Inner Plexiform Layer Parvocellular (IplParvo) parameters
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct RetinaParameters_OPLandIplParvoParameters {
		pub color_mode: bool,
		pub normalise_output: bool,
		pub photoreceptors_local_adaptation_sensitivity: f32,
		pub photoreceptors_temporal_constant: f32,
		pub photoreceptors_spatial_constant: f32,
		pub horizontal_cells_gain: f32,
		pub hcells_temporal_constant: f32,
		pub hcells_spatial_constant: f32,
		pub ganglion_cells_sensitivity: f32,
	}
	
	opencv_type_simple! { crate::bioinspired::RetinaParameters_OPLandIplParvoParameters }
	
	impl RetinaParameters_OPLandIplParvoParameters {
		#[inline]
		pub fn default() -> Result<crate::bioinspired::RetinaParameters_OPLandIplParvoParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// parameter structure that stores the transient events detector setup parameters
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct SegmentationParameters {
		pub threshold_on: f32,
		pub threshold_off: f32,
		/// the time constant of the first order low pass filter, use it to cut high temporal frequencies (noise or fast motion), unit is frames, typical value is 0.5 frame
		pub local_energy_temporal_constant: f32,
		/// the spatial constant of the first order low pass filter, use it to cut high spatial frequencies (noise or thick contours), unit is pixels, typical value is 5 pixel
		pub local_energy_spatial_constant: f32,
		/// local neighborhood energy filtering parameters : the aim is to get information about the energy neighborhood to perform a center surround energy analysis
		pub neighborhood_energy_temporal_constant: f32,
		pub neighborhood_energy_spatial_constant: f32,
		/// context neighborhood energy filtering parameters : the aim is to get information about the energy on a wide neighborhood area to filtered out local effects
		pub context_energy_temporal_constant: f32,
		pub context_energy_spatial_constant: f32,
	}
	
	opencv_type_simple! { crate::bioinspired::SegmentationParameters }
	
	impl SegmentationParameters {
		#[inline]
		pub fn default() -> Result<crate::bioinspired::SegmentationParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_SegmentationParameters_SegmentationParameters(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::bioinspired::TransientAreasSegmentationModule]
	pub trait TransientAreasSegmentationModuleTraitConst: core::AlgorithmTraitConst {
		fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void;
	
		/// write xml/yml formated parameters information
		/// ## Parameters
		/// * fs: : the filename of the xml file that will be open and writen with formatted parameters information
		#[inline]
		fn write(&self, fs: &str) -> Result<()> {
			extern_container_arg!(mut fs);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_write_const_String(self.as_raw_TransientAreasSegmentationModule(), fs.opencv_as_extern_mut(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// write xml/yml formated parameters information
		/// ## Parameters
		/// * fs: : a cv::Filestorage object ready to be filled
		#[inline]
		fn write_to_storage(&self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageR(self.as_raw_TransientAreasSegmentationModule(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::bioinspired::TransientAreasSegmentationModule]
	pub trait TransientAreasSegmentationModuleTrait: core::AlgorithmTrait + crate::bioinspired::TransientAreasSegmentationModuleTraitConst {
		fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void;
	
		/// return the sze of the manage input and output images
		#[inline]
		fn get_size(&mut self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getSize(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		#[inline]
		fn setup_from_file(&mut self, segmentation_parameter_file: &str, apply_default_setup_on_failure: bool) -> Result<()> {
			extern_container_arg!(mut segmentation_parameter_file);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_String_const_bool(self.as_raw_mut_TransientAreasSegmentationModule(), segmentation_parameter_file.opencv_as_extern_mut(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// try to open an XML segmentation parameters file to adjust current segmentation instance setup
		/// 
		/// - if the xml file does not exist, then default setup is applied
		/// - warning, Exceptions are thrown if read XML file is not valid
		/// ## Parameters
		/// * segmentationParameterFile: : the parameters filename
		/// * applyDefaultSetupOnFailure: : set to true if an error must be thrown on error
		/// 
		/// ## Note
		/// This alternative version of [setup_from_file] function uses the following default values for its arguments:
		/// * segmentation_parameter_file: ""
		/// * apply_default_setup_on_failure: true
		#[inline]
		fn setup_from_file_def(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// try to open an XML segmentation parameters file to adjust current segmentation instance setup
		/// 
		/// - if the xml file does not exist, then default setup is applied
		/// - warning, Exceptions are thrown if read XML file is not valid
		/// ## Parameters
		/// * fs: : the open Filestorage which contains segmentation parameters
		/// * applyDefaultSetupOnFailure: : set to true if an error must be thrown on error
		/// 
		/// ## C++ default parameters
		/// * apply_default_setup_on_failure: true
		#[inline]
		fn setup_from_storage(&mut self, fs: &mut core::FileStorage, apply_default_setup_on_failure: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR_const_bool(self.as_raw_mut_TransientAreasSegmentationModule(), fs.as_raw_mut_FileStorage(), apply_default_setup_on_failure, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// try to open an XML segmentation parameters file to adjust current segmentation instance setup
		/// 
		/// - if the xml file does not exist, then default setup is applied
		/// - warning, Exceptions are thrown if read XML file is not valid
		/// ## Parameters
		/// * fs: : the open Filestorage which contains segmentation parameters
		/// * applyDefaultSetupOnFailure: : set to true if an error must be thrown on error
		/// 
		/// ## Note
		/// This alternative version of [setup_from_storage] function uses the following default values for its arguments:
		/// * apply_default_setup_on_failure: true
		#[inline]
		fn setup_from_storage_def(&mut self, fs: &mut core::FileStorage) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR(self.as_raw_mut_TransientAreasSegmentationModule(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// try to open an XML segmentation parameters file to adjust current segmentation instance setup
		/// 
		/// - if the xml file does not exist, then default setup is applied
		/// - warning, Exceptions are thrown if read XML file is not valid
		/// ## Parameters
		/// * newParameters: : a parameters structures updated with the new target configuration
		#[inline]
		fn setup(&mut self, new_parameters: crate::bioinspired::SegmentationParameters) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(self.as_raw_mut_TransientAreasSegmentationModule(), new_parameters.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// return the current parameters setup
		#[inline]
		fn get_parameters(&mut self) -> Result<crate::bioinspired::SegmentationParameters> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getParameters(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// parameters setup display method
		/// ## Returns
		/// a string which contains formatted parameters information
		#[inline]
		fn print_setup(&mut self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_printSetup(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// main processing method, get result using methods getSegmentationPicture()
		/// ## Parameters
		/// * inputToSegment: : the image to process, it must match the instance buffer size !
		/// * channelIndex: : the channel to process in case of multichannel images
		/// 
		/// ## C++ default parameters
		/// * channel_index: 0
		#[inline]
		fn run(&mut self, input_to_segment: &impl core::ToInputArray, channel_index: i32) -> Result<()> {
			input_array_arg!(input_to_segment);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR_const_int(self.as_raw_mut_TransientAreasSegmentationModule(), input_to_segment.as_raw__InputArray(), channel_index, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// main processing method, get result using methods getSegmentationPicture()
		/// ## Parameters
		/// * inputToSegment: : the image to process, it must match the instance buffer size !
		/// * channelIndex: : the channel to process in case of multichannel images
		/// 
		/// ## Note
		/// This alternative version of [run] function uses the following default values for its arguments:
		/// * channel_index: 0
		#[inline]
		fn run_def(&mut self, input_to_segment: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(input_to_segment);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR(self.as_raw_mut_TransientAreasSegmentationModule(), input_to_segment.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// access function
		/// return the last segmentation result: a boolean picture which is resampled between 0 and 255 for a display purpose
		#[inline]
		fn get_segmentation_picture(&mut self, transient_areas: &mut impl core::ToOutputArray) -> Result<()> {
			output_array_arg!(transient_areas);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayR(self.as_raw_mut_TransientAreasSegmentationModule(), transient_areas.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// cleans all the buffers of the instance
		#[inline]
		fn clear_all_buffers(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(self.as_raw_mut_TransientAreasSegmentationModule(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
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
	pub struct TransientAreasSegmentationModule {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TransientAreasSegmentationModule }
	
	impl Drop for TransientAreasSegmentationModule {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_delete(self.as_raw_mut_TransientAreasSegmentationModule()) };
		}
	}
	
	unsafe impl Send for TransientAreasSegmentationModule {}
	
	impl core::AlgorithmTraitConst for TransientAreasSegmentationModule {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for TransientAreasSegmentationModule {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModuleTraitConst for TransientAreasSegmentationModule {
		#[inline] fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModuleTrait for TransientAreasSegmentationModule {
		#[inline] fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TransientAreasSegmentationModule {
		/// allocator
		/// ## Parameters
		/// * inputSize: : size of the images input to segment (output will be the same size)
		#[inline]
		pub fn create(input_size: core::Size) -> Result<core::Ptr<crate::bioinspired::TransientAreasSegmentationModule>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_bioinspired_TransientAreasSegmentationModule_create_Size(input_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::bioinspired::TransientAreasSegmentationModule>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { TransientAreasSegmentationModule, core::Algorithm, cv_bioinspired_TransientAreasSegmentationModule_to_Algorithm }
	
	impl std::fmt::Debug for TransientAreasSegmentationModule {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TransientAreasSegmentationModule")
				.finish()
		}
	}
}

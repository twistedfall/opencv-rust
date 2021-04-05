#[cfg(ocvrs_has_module_aruco)]
pub mod aruco;
#[cfg(ocvrs_has_module_bgsegm)]
pub mod bgsegm;
#[cfg(ocvrs_has_module_bioinspired)]
pub mod bioinspired;
#[cfg(ocvrs_has_module_calib3d)]
pub mod calib3d;
#[cfg(ocvrs_has_module_ccalib)]
pub mod ccalib;
#[cfg(ocvrs_has_module_core)]
pub mod core;
#[cfg(ocvrs_has_module_cvv)]
pub mod cvv;
#[cfg(ocvrs_has_module_dnn)]
pub mod dnn;
#[cfg(ocvrs_has_module_dpm)]
pub mod dpm;
#[cfg(ocvrs_has_module_face)]
pub mod face;
#[cfg(ocvrs_has_module_features2d)]
pub mod features2d;
#[cfg(ocvrs_has_module_flann)]
pub mod flann;
#[cfg(ocvrs_has_module_freetype)]
pub mod freetype;
#[cfg(ocvrs_has_module_fuzzy)]
pub mod fuzzy;
#[cfg(ocvrs_has_module_hdf)]
pub mod hdf;
#[cfg(ocvrs_has_module_highgui)]
pub mod highgui;
#[cfg(ocvrs_has_module_imgcodecs)]
pub mod imgcodecs;
#[cfg(ocvrs_has_module_imgproc)]
pub mod imgproc;
#[cfg(ocvrs_has_module_line_descriptor)]
pub mod line_descriptor;
#[cfg(ocvrs_has_module_ml)]
pub mod ml;
#[cfg(ocvrs_has_module_objdetect)]
pub mod objdetect;
#[cfg(ocvrs_has_module_optflow)]
pub mod optflow;
#[cfg(ocvrs_has_module_phase_unwrapping)]
pub mod phase_unwrapping;
#[cfg(ocvrs_has_module_photo)]
pub mod photo;
#[cfg(ocvrs_has_module_plot)]
pub mod plot;
#[cfg(ocvrs_has_module_rgbd)]
pub mod rgbd;
#[cfg(ocvrs_has_module_saliency)]
pub mod saliency;
#[cfg(ocvrs_has_module_sfm)]
pub mod sfm;
#[cfg(ocvrs_has_module_shape)]
pub mod shape;
#[cfg(ocvrs_has_module_stereo)]
pub mod stereo;
#[cfg(ocvrs_has_module_stitching)]
pub mod stitching;
#[cfg(ocvrs_has_module_structured_light)]
pub mod structured_light;
#[cfg(ocvrs_has_module_superres)]
pub mod superres;
#[cfg(ocvrs_has_module_surface_matching)]
pub mod surface_matching;
#[cfg(ocvrs_has_module_text)]
pub mod text;
#[cfg(ocvrs_has_module_tracking)]
pub mod tracking;
#[cfg(ocvrs_has_module_video)]
pub mod video;
#[cfg(ocvrs_has_module_videoio)]
pub mod videoio;
#[cfg(ocvrs_has_module_videostab)]
pub mod videostab;
#[cfg(ocvrs_has_module_xfeatures2d)]
pub mod xfeatures2d;
#[cfg(ocvrs_has_module_ximgproc)]
pub mod ximgproc;
#[cfg(ocvrs_has_module_xobjdetect)]
pub mod xobjdetect;
#[cfg(ocvrs_has_module_xphoto)]
pub mod xphoto;
pub mod types;
#[doc(hidden)]
pub mod sys;
pub mod hub_prelude {
	#[cfg(ocvrs_has_module_aruco)]
	pub use super::aruco::prelude::*;
	#[cfg(ocvrs_has_module_bgsegm)]
	pub use super::bgsegm::prelude::*;
	#[cfg(ocvrs_has_module_bioinspired)]
	pub use super::bioinspired::prelude::*;
	#[cfg(ocvrs_has_module_calib3d)]
	pub use super::calib3d::prelude::*;
	#[cfg(ocvrs_has_module_ccalib)]
	pub use super::ccalib::prelude::*;
	#[cfg(ocvrs_has_module_core)]
	pub use super::core::prelude::*;
	#[cfg(ocvrs_has_module_cvv)]
	pub use super::cvv::prelude::*;
	#[cfg(ocvrs_has_module_dnn)]
	pub use super::dnn::prelude::*;
	#[cfg(ocvrs_has_module_dpm)]
	pub use super::dpm::prelude::*;
	#[cfg(ocvrs_has_module_face)]
	pub use super::face::prelude::*;
	#[cfg(ocvrs_has_module_features2d)]
	pub use super::features2d::prelude::*;
	#[cfg(ocvrs_has_module_flann)]
	pub use super::flann::prelude::*;
	#[cfg(ocvrs_has_module_freetype)]
	pub use super::freetype::prelude::*;
	#[cfg(ocvrs_has_module_fuzzy)]
	pub use super::fuzzy::prelude::*;
	#[cfg(ocvrs_has_module_hdf)]
	pub use super::hdf::prelude::*;
	#[cfg(ocvrs_has_module_highgui)]
	pub use super::highgui::prelude::*;
	#[cfg(ocvrs_has_module_imgcodecs)]
	pub use super::imgcodecs::prelude::*;
	#[cfg(ocvrs_has_module_imgproc)]
	pub use super::imgproc::prelude::*;
	#[cfg(ocvrs_has_module_line_descriptor)]
	pub use super::line_descriptor::prelude::*;
	#[cfg(ocvrs_has_module_ml)]
	pub use super::ml::prelude::*;
	#[cfg(ocvrs_has_module_objdetect)]
	pub use super::objdetect::prelude::*;
	#[cfg(ocvrs_has_module_optflow)]
	pub use super::optflow::prelude::*;
	#[cfg(ocvrs_has_module_phase_unwrapping)]
	pub use super::phase_unwrapping::prelude::*;
	#[cfg(ocvrs_has_module_photo)]
	pub use super::photo::prelude::*;
	#[cfg(ocvrs_has_module_plot)]
	pub use super::plot::prelude::*;
	#[cfg(ocvrs_has_module_rgbd)]
	pub use super::rgbd::prelude::*;
	#[cfg(ocvrs_has_module_saliency)]
	pub use super::saliency::prelude::*;
	#[cfg(ocvrs_has_module_sfm)]
	pub use super::sfm::prelude::*;
	#[cfg(ocvrs_has_module_shape)]
	pub use super::shape::prelude::*;
	#[cfg(ocvrs_has_module_stereo)]
	pub use super::stereo::prelude::*;
	#[cfg(ocvrs_has_module_stitching)]
	pub use super::stitching::prelude::*;
	#[cfg(ocvrs_has_module_structured_light)]
	pub use super::structured_light::prelude::*;
	#[cfg(ocvrs_has_module_superres)]
	pub use super::superres::prelude::*;
	#[cfg(ocvrs_has_module_surface_matching)]
	pub use super::surface_matching::prelude::*;
	#[cfg(ocvrs_has_module_text)]
	pub use super::text::prelude::*;
	#[cfg(ocvrs_has_module_tracking)]
	pub use super::tracking::prelude::*;
	#[cfg(ocvrs_has_module_video)]
	pub use super::video::prelude::*;
	#[cfg(ocvrs_has_module_videoio)]
	pub use super::videoio::prelude::*;
	#[cfg(ocvrs_has_module_videostab)]
	pub use super::videostab::prelude::*;
	#[cfg(ocvrs_has_module_xfeatures2d)]
	pub use super::xfeatures2d::prelude::*;
	#[cfg(ocvrs_has_module_ximgproc)]
	pub use super::ximgproc::prelude::*;
	#[cfg(ocvrs_has_module_xobjdetect)]
	pub use super::xobjdetect::prelude::*;
	#[cfg(ocvrs_has_module_xphoto)]
	pub use super::xphoto::prelude::*;
}

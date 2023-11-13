#[cfg(ocvrs_has_module_alphamat)]
include!(concat!(env!("OUT_DIR"), "/opencv/alphamat.rs"));
#[cfg(ocvrs_has_module_aruco)]
include!(concat!(env!("OUT_DIR"), "/opencv/aruco.rs"));
#[cfg(ocvrs_has_module_bgsegm)]
include!(concat!(env!("OUT_DIR"), "/opencv/bgsegm.rs"));
#[cfg(ocvrs_has_module_bioinspired)]
include!(concat!(env!("OUT_DIR"), "/opencv/bioinspired.rs"));
#[cfg(ocvrs_has_module_calib3d)]
include!(concat!(env!("OUT_DIR"), "/opencv/calib3d.rs"));
#[cfg(ocvrs_has_module_ccalib)]
include!(concat!(env!("OUT_DIR"), "/opencv/ccalib.rs"));
#[cfg(ocvrs_has_module_core)]
include!(concat!(env!("OUT_DIR"), "/opencv/core.rs"));
#[cfg(ocvrs_has_module_cudaarithm)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudaarithm.rs"));
#[cfg(ocvrs_has_module_cudabgsegm)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudabgsegm.rs"));
#[cfg(ocvrs_has_module_cudacodec)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudacodec.rs"));
#[cfg(ocvrs_has_module_cudafeatures2d)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudafeatures2d.rs"));
#[cfg(ocvrs_has_module_cudafilters)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudafilters.rs"));
#[cfg(ocvrs_has_module_cudaimgproc)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudaimgproc.rs"));
#[cfg(ocvrs_has_module_cudaobjdetect)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudaobjdetect.rs"));
#[cfg(ocvrs_has_module_cudaoptflow)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudaoptflow.rs"));
#[cfg(ocvrs_has_module_cudastereo)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudastereo.rs"));
#[cfg(ocvrs_has_module_cudawarping)]
include!(concat!(env!("OUT_DIR"), "/opencv/cudawarping.rs"));
#[cfg(ocvrs_has_module_cvv)]
include!(concat!(env!("OUT_DIR"), "/opencv/cvv.rs"));
#[cfg(ocvrs_has_module_dnn)]
include!(concat!(env!("OUT_DIR"), "/opencv/dnn.rs"));
#[cfg(ocvrs_has_module_dnn_superres)]
include!(concat!(env!("OUT_DIR"), "/opencv/dnn_superres.rs"));
#[cfg(ocvrs_has_module_dpm)]
include!(concat!(env!("OUT_DIR"), "/opencv/dpm.rs"));
#[cfg(ocvrs_has_module_face)]
include!(concat!(env!("OUT_DIR"), "/opencv/face.rs"));
#[cfg(ocvrs_has_module_features2d)]
include!(concat!(env!("OUT_DIR"), "/opencv/features2d.rs"));
#[cfg(ocvrs_has_module_flann)]
include!(concat!(env!("OUT_DIR"), "/opencv/flann.rs"));
#[cfg(ocvrs_has_module_freetype)]
include!(concat!(env!("OUT_DIR"), "/opencv/freetype.rs"));
#[cfg(ocvrs_has_module_fuzzy)]
include!(concat!(env!("OUT_DIR"), "/opencv/fuzzy.rs"));
#[cfg(ocvrs_has_module_gapi)]
include!(concat!(env!("OUT_DIR"), "/opencv/gapi.rs"));
#[cfg(ocvrs_has_module_hdf)]
include!(concat!(env!("OUT_DIR"), "/opencv/hdf.rs"));
#[cfg(ocvrs_has_module_hfs)]
include!(concat!(env!("OUT_DIR"), "/opencv/hfs.rs"));
#[cfg(ocvrs_has_module_highgui)]
include!(concat!(env!("OUT_DIR"), "/opencv/highgui.rs"));
#[cfg(ocvrs_has_module_img_hash)]
include!(concat!(env!("OUT_DIR"), "/opencv/img_hash.rs"));
#[cfg(ocvrs_has_module_imgcodecs)]
include!(concat!(env!("OUT_DIR"), "/opencv/imgcodecs.rs"));
#[cfg(ocvrs_has_module_imgproc)]
include!(concat!(env!("OUT_DIR"), "/opencv/imgproc.rs"));
#[cfg(ocvrs_has_module_intensity_transform)]
include!(concat!(env!("OUT_DIR"), "/opencv/intensity_transform.rs"));
#[cfg(ocvrs_has_module_line_descriptor)]
include!(concat!(env!("OUT_DIR"), "/opencv/line_descriptor.rs"));
#[cfg(ocvrs_has_module_mcc)]
include!(concat!(env!("OUT_DIR"), "/opencv/mcc.rs"));
#[cfg(ocvrs_has_module_ml)]
include!(concat!(env!("OUT_DIR"), "/opencv/ml.rs"));
#[cfg(ocvrs_has_module_objdetect)]
include!(concat!(env!("OUT_DIR"), "/opencv/objdetect.rs"));
#[cfg(ocvrs_has_module_optflow)]
include!(concat!(env!("OUT_DIR"), "/opencv/optflow.rs"));
#[cfg(ocvrs_has_module_ovis)]
include!(concat!(env!("OUT_DIR"), "/opencv/ovis.rs"));
#[cfg(ocvrs_has_module_phase_unwrapping)]
include!(concat!(env!("OUT_DIR"), "/opencv/phase_unwrapping.rs"));
#[cfg(ocvrs_has_module_photo)]
include!(concat!(env!("OUT_DIR"), "/opencv/photo.rs"));
#[cfg(ocvrs_has_module_plot)]
include!(concat!(env!("OUT_DIR"), "/opencv/plot.rs"));
#[cfg(ocvrs_has_module_quality)]
include!(concat!(env!("OUT_DIR"), "/opencv/quality.rs"));
#[cfg(ocvrs_has_module_rapid)]
include!(concat!(env!("OUT_DIR"), "/opencv/rapid.rs"));
#[cfg(ocvrs_has_module_rgbd)]
include!(concat!(env!("OUT_DIR"), "/opencv/rgbd.rs"));
#[cfg(ocvrs_has_module_saliency)]
include!(concat!(env!("OUT_DIR"), "/opencv/saliency.rs"));
#[cfg(ocvrs_has_module_sfm)]
include!(concat!(env!("OUT_DIR"), "/opencv/sfm.rs"));
#[cfg(ocvrs_has_module_shape)]
include!(concat!(env!("OUT_DIR"), "/opencv/shape.rs"));
#[cfg(ocvrs_has_module_stereo)]
include!(concat!(env!("OUT_DIR"), "/opencv/stereo.rs"));
#[cfg(ocvrs_has_module_stitching)]
include!(concat!(env!("OUT_DIR"), "/opencv/stitching.rs"));
#[cfg(ocvrs_has_module_structured_light)]
include!(concat!(env!("OUT_DIR"), "/opencv/structured_light.rs"));
#[cfg(ocvrs_has_module_superres)]
include!(concat!(env!("OUT_DIR"), "/opencv/superres.rs"));
#[cfg(ocvrs_has_module_surface_matching)]
include!(concat!(env!("OUT_DIR"), "/opencv/surface_matching.rs"));
#[cfg(ocvrs_has_module_text)]
include!(concat!(env!("OUT_DIR"), "/opencv/text.rs"));
#[cfg(ocvrs_has_module_tracking)]
include!(concat!(env!("OUT_DIR"), "/opencv/tracking.rs"));
#[cfg(ocvrs_has_module_video)]
include!(concat!(env!("OUT_DIR"), "/opencv/video.rs"));
#[cfg(ocvrs_has_module_videoio)]
include!(concat!(env!("OUT_DIR"), "/opencv/videoio.rs"));
#[cfg(ocvrs_has_module_videostab)]
include!(concat!(env!("OUT_DIR"), "/opencv/videostab.rs"));
#[cfg(ocvrs_has_module_viz)]
include!(concat!(env!("OUT_DIR"), "/opencv/viz.rs"));
#[cfg(ocvrs_has_module_wechat_qrcode)]
include!(concat!(env!("OUT_DIR"), "/opencv/wechat_qrcode.rs"));
#[cfg(ocvrs_has_module_xfeatures2d)]
include!(concat!(env!("OUT_DIR"), "/opencv/xfeatures2d.rs"));
#[cfg(ocvrs_has_module_ximgproc)]
include!(concat!(env!("OUT_DIR"), "/opencv/ximgproc.rs"));
#[cfg(ocvrs_has_module_xobjdetect)]
include!(concat!(env!("OUT_DIR"), "/opencv/xobjdetect.rs"));
#[cfg(ocvrs_has_module_xphoto)]
include!(concat!(env!("OUT_DIR"), "/opencv/xphoto.rs"));
pub mod types {
include!(concat!(env!("OUT_DIR"), "/opencv/types.rs"));
}
#[doc(hidden)]
pub mod sys {
include!(concat!(env!("OUT_DIR"), "/opencv/sys.rs"));
}
pub mod hub_prelude {
	#[cfg(ocvrs_has_module_alphamat)]
	pub use super::alphamat::prelude::*;
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
	#[cfg(ocvrs_has_module_cudaarithm)]
	pub use super::cudaarithm::prelude::*;
	#[cfg(ocvrs_has_module_cudabgsegm)]
	pub use super::cudabgsegm::prelude::*;
	#[cfg(ocvrs_has_module_cudacodec)]
	pub use super::cudacodec::prelude::*;
	#[cfg(ocvrs_has_module_cudafeatures2d)]
	pub use super::cudafeatures2d::prelude::*;
	#[cfg(ocvrs_has_module_cudafilters)]
	pub use super::cudafilters::prelude::*;
	#[cfg(ocvrs_has_module_cudaimgproc)]
	pub use super::cudaimgproc::prelude::*;
	#[cfg(ocvrs_has_module_cudaobjdetect)]
	pub use super::cudaobjdetect::prelude::*;
	#[cfg(ocvrs_has_module_cudaoptflow)]
	pub use super::cudaoptflow::prelude::*;
	#[cfg(ocvrs_has_module_cudastereo)]
	pub use super::cudastereo::prelude::*;
	#[cfg(ocvrs_has_module_cudawarping)]
	pub use super::cudawarping::prelude::*;
	#[cfg(ocvrs_has_module_cvv)]
	pub use super::cvv::prelude::*;
	#[cfg(ocvrs_has_module_dnn)]
	pub use super::dnn::prelude::*;
	#[cfg(ocvrs_has_module_dnn_superres)]
	pub use super::dnn_superres::prelude::*;
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
	#[cfg(ocvrs_has_module_gapi)]
	pub use super::gapi::prelude::*;
	#[cfg(ocvrs_has_module_hdf)]
	pub use super::hdf::prelude::*;
	#[cfg(ocvrs_has_module_hfs)]
	pub use super::hfs::prelude::*;
	#[cfg(ocvrs_has_module_highgui)]
	pub use super::highgui::prelude::*;
	#[cfg(ocvrs_has_module_img_hash)]
	pub use super::img_hash::prelude::*;
	#[cfg(ocvrs_has_module_imgcodecs)]
	pub use super::imgcodecs::prelude::*;
	#[cfg(ocvrs_has_module_imgproc)]
	pub use super::imgproc::prelude::*;
	#[cfg(ocvrs_has_module_intensity_transform)]
	pub use super::intensity_transform::prelude::*;
	#[cfg(ocvrs_has_module_line_descriptor)]
	pub use super::line_descriptor::prelude::*;
	#[cfg(ocvrs_has_module_mcc)]
	pub use super::mcc::prelude::*;
	#[cfg(ocvrs_has_module_ml)]
	pub use super::ml::prelude::*;
	#[cfg(ocvrs_has_module_objdetect)]
	pub use super::objdetect::prelude::*;
	#[cfg(ocvrs_has_module_optflow)]
	pub use super::optflow::prelude::*;
	#[cfg(ocvrs_has_module_ovis)]
	pub use super::ovis::prelude::*;
	#[cfg(ocvrs_has_module_phase_unwrapping)]
	pub use super::phase_unwrapping::prelude::*;
	#[cfg(ocvrs_has_module_photo)]
	pub use super::photo::prelude::*;
	#[cfg(ocvrs_has_module_plot)]
	pub use super::plot::prelude::*;
	#[cfg(ocvrs_has_module_quality)]
	pub use super::quality::prelude::*;
	#[cfg(ocvrs_has_module_rapid)]
	pub use super::rapid::prelude::*;
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
	#[cfg(ocvrs_has_module_viz)]
	pub use super::viz::prelude::*;
	#[cfg(ocvrs_has_module_wechat_qrcode)]
	pub use super::wechat_qrcode::prelude::*;
	#[cfg(ocvrs_has_module_xfeatures2d)]
	pub use super::xfeatures2d::prelude::*;
	#[cfg(ocvrs_has_module_ximgproc)]
	pub use super::ximgproc::prelude::*;
	#[cfg(ocvrs_has_module_xobjdetect)]
	pub use super::xobjdetect::prelude::*;
	#[cfg(ocvrs_has_module_xphoto)]
	pub use super::xphoto::prelude::*;
}

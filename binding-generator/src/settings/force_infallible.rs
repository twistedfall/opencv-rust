use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::FuncId;

pub static FORCE_INFALLIBLE: Lazy<HashSet<FuncId>> = Lazy::new(|| {
	HashSet::from([
		// just returns static/constant data
		FuncId::new_mut("cv::noArray", []),
		FuncId::new_mut("cv::getVersionMajor", []),
		FuncId::new_mut("cv::getVersionMinor", []),
		FuncId::new_mut("cv::getVersionRevision", []),
		// not doing anything that can cause an exception
		FuncId::new_const("cv::Mat::empty", []),
		FuncId::new_const("cv::Mat::total", []),
		FuncId::new_const("cv::Mat::isContinuous", []),
		FuncId::new_const("cv::Mat::isSubmatrix", []),
		FuncId::new_const("cv::Mat::elemSize1", []),
		FuncId::new_const("cv::Mat::type", []),
		FuncId::new_const("cv::Mat::depth", []),
		FuncId::new_const("cv::Mat::channels", []),
		FuncId::new_const("cv::UMat::empty", []),
		FuncId::new_const("cv::UMat::total", []),
		FuncId::new_const("cv::UMat::isContinuous", []),
		FuncId::new_const("cv::UMat::isSubmatrix", []),
		FuncId::new_const("cv::UMat::elemSize1", []),
		FuncId::new_const("cv::UMat::type", []),
		FuncId::new_const("cv::UMat::depth", []),
		FuncId::new_const("cv::UMat::channels", []),
		FuncId::new_const("cv::SparseMat::elemSize", []),
		FuncId::new_const("cv::SparseMat::elemSize1", []),
		FuncId::new_const("cv::SparseMat::type", []),
		FuncId::new_const("cv::SparseMat::depth", []),
		FuncId::new_const("cv::SparseMat::channels", []),
		// marked CV_NOEXCEPT since OpenCV 4.5.2, propagate those changes to earlier versions
		FuncId::new_mut("cv::Mat::Mat", []),
		FuncId::new_mut("cv::MatSize::MatSize", ["_p"]),
		FuncId::new_const("cv::MatSize::dims", []),
		FuncId::new_const("cv::MatSize::operator const int *", []),
		FuncId::new_mut("cv::MatStep::MatStep", []),
		FuncId::new_mut("cv::MatStep::operator[]", ["i"]),
		FuncId::new_mut("cv::UMat::UMat", ["usageFlags"]),
		FuncId::new_mut("cv::ocl::Context::Context", []),
		FuncId::new_mut("cv::ocl::Device::Device", []),
		FuncId::new_mut("cv::ocl::Image2D::Image2D", []),
		FuncId::new_mut("cv::ocl::Kernel::Kernel", []),
		FuncId::new_mut("cv::ocl::KernelArg::KernelArg", []),
		FuncId::new_mut("cv::ocl::Platform::Platform", []),
		FuncId::new_mut("cv::ocl::PlatformInfo::PlatformInfo", []),
		FuncId::new_mut("cv::ocl::Program::Program", []),
		FuncId::new_mut("cv::ocl::ProgramSource::ProgramSource", []),
		FuncId::new_mut("cv::ocl::Queue::Queue", []),
	])
});

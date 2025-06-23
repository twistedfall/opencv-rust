use std::collections::HashMap;

use crate::func::FuncMatcher;
use crate::SupportedModule;

pub type ForceInfallible = FuncMatcher<'static, ()>;

pub fn force_infallible_factory(module: SupportedModule) -> ForceInfallible {
	match module {
		SupportedModule::Core => core_factory(),
		_ => ForceInfallible::empty(),
	}
}

fn core_factory() -> ForceInfallible {
	FuncMatcher::create(HashMap::from([
		// just returns static/constant data
		("cv::noArray", vec![(pred!(mut, []), ())]),
		("cv::getVersionMajor", vec![(pred!(mut, []), ())]),
		("cv::getVersionMinor", vec![(pred!(mut, []), ())]),
		("cv::getVersionRevision", vec![(pred!(mut, []), ())]),
		// not doing anything that can cause an exception
		("cv::Mat::empty", vec![(pred!(const, []), ())]),
		("cv::Mat::total", vec![(pred!(const, []), ())]),
		("cv::Mat::isContinuous", vec![(pred!(const, []), ())]),
		("cv::Mat::isSubmatrix", vec![(pred!(const, []), ())]),
		("cv::Mat::elemSize1", vec![(pred!(const, []), ())]),
		("cv::Mat::type", vec![(pred!(const, []), ())]),
		("cv::Mat::depth", vec![(pred!(const, []), ())]),
		("cv::Mat::channels", vec![(pred!(const, []), ())]),
		("cv::UMat::empty", vec![(pred!(const, []), ())]),
		("cv::UMat::total", vec![(pred!(const, []), ())]),
		("cv::UMat::isContinuous", vec![(pred!(const, []), ())]),
		("cv::UMat::isSubmatrix", vec![(pred!(const, []), ())]),
		("cv::UMat::elemSize1", vec![(pred!(const, []), ())]),
		("cv::UMat::type", vec![(pred!(const, []), ())]),
		("cv::UMat::depth", vec![(pred!(const, []), ())]),
		("cv::UMat::channels", vec![(pred!(const, []), ())]),
		("cv::SparseMat::elemSize", vec![(pred!(const, []), ())]),
		("cv::SparseMat::elemSize1", vec![(pred!(const, []), ())]),
		("cv::SparseMat::type", vec![(pred!(const, []), ())]),
		("cv::SparseMat::depth", vec![(pred!(const, []), ())]),
		("cv::SparseMat::channels", vec![(pred!(const, []), ())]),
		// marked CV_NOEXCEPT since OpenCV 4.5.2, propagate those changes to earlier versions
		("cv::Mat::Mat", vec![(pred!(mut, []), ())]),
		("cv::MatSize::MatSize", vec![(pred!(mut, ["_p"]), ())]),
		("cv::MatSize::dims", vec![(pred!(const, []), ())]),
		("cv::MatSize::operator const int*", vec![(pred!(const, []), ())]),
		("cv::MatStep::MatStep", vec![(pred!(mut, []), ())]),
		("cv::MatStep::operator[]", vec![(pred!(mut, ["i"]), ())]),
		("cv::UMat::UMat", vec![(pred!(mut, ["usageFlags"]), ())]),
		("cv::ocl::Context::Context", vec![(pred!(mut, []), ())]),
		("cv::ocl::Device::Device", vec![(pred!(mut, []), ())]),
		("cv::ocl::Image2D::Image2D", vec![(pred!(mut, []), ())]),
		("cv::ocl::Kernel::Kernel", vec![(pred!(mut, []), ())]),
		("cv::ocl::KernelArg::KernelArg", vec![(pred!(mut, []), ())]),
		("cv::ocl::Platform::Platform", vec![(pred!(mut, []), ())]),
		("cv::ocl::PlatformInfo::PlatformInfo", vec![(pred!(mut, []), ())]),
		("cv::ocl::Program::Program", vec![(pred!(mut, []), ())]),
		("cv::ocl::ProgramSource::ProgramSource", vec![(pred!(mut, []), ())]),
		("cv::ocl::Queue::Queue", vec![(pred!(mut, []), ())]),
	]))
}

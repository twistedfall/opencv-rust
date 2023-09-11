use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::FuncId;

/// set of functions that should have unsafe in their declaration, element is Func.identifier()
pub static FUNC_UNSAFE: Lazy<HashSet<FuncId>> = Lazy::new(|| {
	HashSet::from([
		// allocates uninitialized memory
		FuncId::new("cv::Mat::Mat", ["size", "type"]),
		FuncId::new("cv::Mat::Mat", ["sizes", "type"]),
		FuncId::new("cv::Mat::Mat", ["ndims", "sizes", "type"]),
		FuncId::new("cv::Mat::Mat", ["rows", "cols", "type"]),
		FuncId::new("cv::Mat::create", ["size", "type"]),
		FuncId::new("cv::Mat::create", ["sizes", "type"]),
		FuncId::new("cv::Mat::create", ["ndims", "sizes", "type"]),
		FuncId::new("cv::Mat::create", ["rows", "cols", "type"]),
		FuncId::new("cv::UMat::UMat", ["size", "type", "usageFlags"]),
		FuncId::new("cv::UMat::UMat", ["ndims", "sizes", "type", "usageFlags"]),
		FuncId::new("cv::UMat::UMat", ["rows", "cols", "type", "usageFlags"]),
		FuncId::new("cv::UMat::create", ["size", "type", "usageFlags"]),
		FuncId::new("cv::UMat::create", ["size", "type", "usageFlags"]),
		FuncId::new("cv::UMat::create", ["ndims", "sizes", "type", "usageFlags"]),
		FuncId::new("cv::UMat::create", ["sizes", "type", "usageFlags"]),
		FuncId::new("cv::UMat::create", ["rows", "cols", "type", "usageFlags"]),
		FuncId::new("cv::_OutputArray::createSameSize", ["arr", "mtype"]),
		// pointer to internal data
		FuncId::new("cv::dnn::Dict::ptr", ["key"]),
		// takes reference and stores it for the lifetime of an object (fixme: add lifetime management)
		FuncId::new("cv::cuda::GpuMat::GpuMat", ["allocator"]),
		FuncId::new("cv::cuda::GpuMat::GpuMat", ["size", "type", "allocator"]),
		FuncId::new("cv::cuda::GpuMat::GpuMat", ["size", "type", "s", "allocator"]),
		FuncId::new("cv::cuda::GpuMat::GpuMat", ["arr", "allocator"]),
		FuncId::new("cv::cuda::GpuMat::GpuMat", ["rows", "cols", "type", "allocator"]),
		FuncId::new("cv::cuda::GpuMat::GpuMat", ["rows", "cols", "type", "s", "allocator"]),
		FuncId::new("cv::cuda::GpuMat::setAllocator", ["val"]),
		FuncId::new("cv::cuda::GpuMat::setDefaultAllocator", ["allocator"]), // fixme, should take 'static
	])
});

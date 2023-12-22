use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::FuncId;

/// set of functions that should have unsafe in their declaration, element is Func.identifier()
pub static FUNC_UNSAFE: Lazy<HashSet<FuncId>> = Lazy::new(|| {
	HashSet::from([
		// allocates uninitialized memory
		FuncId::new_mut("cv::Mat::Mat", ["size", "type"]),
		FuncId::new_mut("cv::Mat::Mat", ["sizes", "type"]),
		FuncId::new_mut("cv::Mat::Mat", ["ndims", "sizes", "type"]),
		FuncId::new_mut("cv::Mat::Mat", ["rows", "cols", "type"]),
		FuncId::new_mut("cv::Mat::create", ["size", "type"]),
		FuncId::new_mut("cv::Mat::create", ["sizes", "type"]),
		FuncId::new_mut("cv::Mat::create", ["ndims", "sizes", "type"]),
		FuncId::new_mut("cv::Mat::create", ["rows", "cols", "type"]),
		FuncId::new_mut("cv::UMat::UMat", ["size", "type", "usageFlags"]),
		FuncId::new_mut("cv::UMat::UMat", ["ndims", "sizes", "type", "usageFlags"]),
		FuncId::new_mut("cv::UMat::UMat", ["rows", "cols", "type", "usageFlags"]),
		FuncId::new_mut("cv::UMat::create", ["size", "type", "usageFlags"]),
		FuncId::new_mut("cv::UMat::create", ["size", "type", "usageFlags"]),
		FuncId::new_mut("cv::UMat::create", ["ndims", "sizes", "type", "usageFlags"]),
		FuncId::new_mut("cv::UMat::create", ["sizes", "type", "usageFlags"]),
		FuncId::new_mut("cv::UMat::create", ["rows", "cols", "type", "usageFlags"]),
		FuncId::new_const("cv::_OutputArray::createSameSize", ["arr", "mtype"]),
		// manual manipulation of reference counter
		FuncId::new_mut("cv::Mat::addref", []),
		FuncId::new_mut("cv::Mat::release", []),
		FuncId::new_mut("cv::SparseMat::addref", []),
		FuncId::new_mut("cv::SparseMat::release", []),
		FuncId::new_mut("cv::UMat::addref", []),
		FuncId::new_mut("cv::UMat::release", []),
		// pointer to internal data
		FuncId::new_const("cv::dnn::Dict::ptr", ["key"]),
		FuncId::new_mut("cv::dnn::Dict::ptr", ["key"]),
		// takes reference and stores it for the lifetime of an object (fixme: add lifetime management)
		FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["allocator"]),
		FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["size", "type", "allocator"]),
		FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["size", "type", "s", "allocator"]),
		FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["arr", "allocator"]),
		FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["rows", "cols", "type", "allocator"]),
		FuncId::new_mut("cv::cuda::GpuMat::GpuMat", ["rows", "cols", "type", "s", "allocator"]),
		FuncId::new_mut("cv::cuda::GpuMat::setAllocator", ["val"]),
		FuncId::new_mut("cv::cuda::GpuMat::setDefaultAllocator", ["allocator"]), // fixme, should take 'static
	])
});

// fixme, covers mat from gpumat which it shouldn't
// FuncId::new_mut("cv::Mat::Mat", ["m"]),

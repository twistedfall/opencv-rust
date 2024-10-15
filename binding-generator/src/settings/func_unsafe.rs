use std::collections::HashMap;

use crate::func::FuncMatcher;

pub type FuncUnsafe = FuncMatcher<'static, ()>;

/// set of functions that should have unsafe in their declaration, element is Func.identifier()
pub fn func_unsafe_factory(module: &str) -> FuncUnsafe {
	match module {
		"core" => core_factory(),
		"dnn" => dnn_factory(),
		_ => FuncUnsafe::empty(),
	}
}

fn core_factory() -> FuncUnsafe {
	FuncMatcher::create(HashMap::from([
		// allocates uninitialized memory
		(
			"cv::Mat::Mat",
			vec![
				(pred!(mut, ["size", "type"]), ()),
				(pred!(mut, ["sizes", "type"]), ()),
				(pred!(mut, ["ndims", "sizes", "type"]), ()),
				(pred!(mut, ["rows", "cols", "type"]), ()),
			],
		),
		(
			"cv::Mat::create",
			vec![
				(pred!(mut, ["size", "type"]), ()),
				(pred!(mut, ["sizes", "type"]), ()),
				(pred!(mut, ["ndims", "sizes", "type"]), ()),
				(pred!(mut, ["rows", "cols", "type"]), ()),
			],
		),
		(
			"cv::UMat::UMat",
			vec![
				(pred!(mut, ["size", "type", "usageFlags"]), ()),
				(pred!(mut, ["ndims", "sizes", "type", "usageFlags"]), ()),
				(pred!(mut, ["rows", "cols", "type", "usageFlags"]), ()),
			],
		),
		(
			"cv::UMat::create",
			vec![
				(pred!(mut, ["size", "type", "usageFlags"]), ()),
				(pred!(mut, ["size", "type", "usageFlags"]), ()),
				(pred!(mut, ["ndims", "sizes", "type", "usageFlags"]), ()),
				(pred!(mut, ["sizes", "type", "usageFlags"]), ()),
				(pred!(mut, ["rows", "cols", "type", "usageFlags"]), ()),
			],
		),
		("cv::_OutputArray::createSameSize", vec![(pred!(const, ["arr", "mtype"]), ())]),
		// manual manipulation of reference counter
		("cv::Mat::addref", vec![(pred!(mut, []), ())]),
		("cv::Mat::release", vec![(pred!(mut, []), ())]),
		("cv::SparseMat::addref", vec![(pred!(mut, []), ())]),
		("cv::SparseMat::release", vec![(pred!(mut, []), ())]),
		("cv::UMat::addref", vec![(pred!(mut, []), ())]),
		("cv::UMat::release", vec![(pred!(mut, []), ())]),
		// takes reference and stores it for the lifetime of an object (fixme: add lifetime management)
		(
			"cv::cuda::GpuMat::GpuMat",
			vec![
				(pred!(mut, ["allocator"]), ()),
				(pred!(mut, ["size", "type", "allocator"]), ()),
				(pred!(mut, ["size", "type", "s", "allocator"]), ()),
				(pred!(mut, ["arr", "allocator"]), ()),
				(pred!(mut, ["rows", "cols", "type", "allocator"]), ()),
				(pred!(mut, ["rows", "cols", "type", "s", "allocator"]), ()),
			],
		),
	]))
}

fn dnn_factory() -> FuncUnsafe {
	FuncMatcher::create(HashMap::from([
		// pointer to internal data
		(
			"cv::dnn::Dict::ptr",
			vec![(pred!(const, ["key"]), ()), (pred!(mut, ["key"]), ())],
		),
	]))
}

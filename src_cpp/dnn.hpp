#include "ocvrs_common.hpp"
#include <opencv2/dnn.hpp>
#if CV_VERSION_MAJOR == 4
	#include <opencv2/dnn/version.hpp>
#endif
#include <opencv2/dnn/all_layers.hpp>
#include <opencv2/dnn/shape_utils.hpp>

// see ocvrs_resolve_types.hpp
namespace ocvrs_resolve_types {
	// for specializing cv::dnn::Dict::set
	typedef cv::dnn::DictValue dnn1;

	// for specializing cv::dnn::DictValue::get
	typedef cv::String dnn2;
	typedef int64_t dnn3;
}

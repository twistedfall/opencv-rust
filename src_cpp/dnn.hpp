// strip dnn experimental ns when generating headers
#ifdef OCVRS_PARSING_HEADERS
	#define CV_DNN_DONT_ADD_EXPERIMENTAL_NS
	#define CV_DNN_DONT_ADD_INLINE_NS
#endif

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

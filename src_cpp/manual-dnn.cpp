#include "dnn.hpp"

template struct Result<void*>;

extern "C" {
	Result<void*> cv_dnn_LayerParams_LayerParams() {
		try {
			return Ok<void*>(new cv::dnn::LayerParams());
		} OCVRS_CATCH(Result<void*>)
	}
}

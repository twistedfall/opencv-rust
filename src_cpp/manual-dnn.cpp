#include "dnn.hpp"

template struct Result<void*>;

extern "C" {
	void cv_dnn_LayerParams_LayerParams(Result<void*>* ocvrs_return) {
		try {
			return Ok<void*>(new cv::dnn::LayerParams(), ocvrs_return);
		} OCVRS_CATCH(Result<void*>)
	}
}

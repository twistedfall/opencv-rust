#include "ocvrs_common.hpp"
#include <opencv2/features2d.hpp>

template struct Result<void*>;

extern "C" {
	void cv_ORB_create(Result<void*>* ocvrs_return) {
		try {
			return Ok<void*>(new cv::Ptr<cv::ORB>(cv::ORB::create()), ocvrs_return);
		} OCVRS_CATCH(Result<void*>)
	}
}

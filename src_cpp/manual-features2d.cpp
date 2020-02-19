#include "common.hpp"
#include <opencv2/features2d.hpp>

template struct Result<void*>;

extern "C" {
	Result<void*> cv_ORB_create() {
		try {
			return Ok<void*>(new cv::Ptr<cv::ORB>(cv::ORB::create()));
		} OCVRS_CATCH(Result<void*>)
	}
}

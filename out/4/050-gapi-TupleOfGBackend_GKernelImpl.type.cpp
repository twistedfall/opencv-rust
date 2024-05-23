extern "C" {
	// std::pair<cv::gapi::GBackend, cv::GKernelImpl>::new(TraitClass, TraitClass) generated
	// ("std::pair<cv::gapi::GBackend, cv::GKernelImpl>::new", vec![(pred!(const, ["arg", "arg_1"], ["cv::gapi::GBackend", "cv::GKernelImpl"]), _)]),
	std::pair<cv::gapi::GBackend, cv::GKernelImpl>* std_pairLcv_gapi_GBackend__cv_GKernelImplG_new_const_GBackend_GKernelImpl(cv::gapi::GBackend* arg, cv::GKernelImpl* arg_1) {
			std::pair<cv::gapi::GBackend, cv::GKernelImpl>* ret = new std::pair<cv::gapi::GBackend, cv::GKernelImpl>(*arg, *arg_1);
			return ret;
	}

	// std::pair<cv::gapi::GBackend, cv::GKernelImpl>::get_0() generated
	// ("std::pair<cv::gapi::GBackend, cv::GKernelImpl>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_gapi_GBackend__cv_GKernelImplG_get_0_const(const std::pair<cv::gapi::GBackend, cv::GKernelImpl>* instance, cv::gapi::GBackend** ocvrs_return) {
			cv::gapi::GBackend ret = std::get<0>(*instance);
			*ocvrs_return = new cv::gapi::GBackend(ret);
	}

	// std::pair<cv::gapi::GBackend, cv::GKernelImpl>::get_1() generated
	// ("std::pair<cv::gapi::GBackend, cv::GKernelImpl>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_gapi_GBackend__cv_GKernelImplG_get_1_const(const std::pair<cv::gapi::GBackend, cv::GKernelImpl>* instance, cv::GKernelImpl** ocvrs_return) {
			cv::GKernelImpl ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GKernelImpl(ret);
	}

	// std::pair<cv::gapi::GBackend, cv::GKernelImpl>::delete() generated
	// ("std::pair<cv::gapi::GBackend, cv::GKernelImpl>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLcv_gapi_GBackend__cv_GKernelImplG_delete(std::pair<cv::gapi::GBackend, cv::GKernelImpl>* instance) {
			delete instance;
	}

}


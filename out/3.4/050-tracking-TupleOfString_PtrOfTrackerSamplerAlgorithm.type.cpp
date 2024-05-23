extern "C" {
	// std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::new(InString, CppPassByVoidPtr) generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::new", vec![(pred!(const, ["arg", "arg_1"], ["cv::String", "cv::Ptr<cv::TrackerSamplerAlgorithm>"]), _)]),
	std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_new_const_String_PtrLTrackerSamplerAlgorithmG(const char* arg, cv::Ptr<cv::TrackerSamplerAlgorithm>* arg_1) {
			std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* ret = new std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>(cv::String(arg), *arg_1);
			return ret;
	}

	// std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::get_0() generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_get_0_const(const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* instance, void** ocvrs_return) {
			cv::String ret = std::get<0>(*instance);
			*ocvrs_return = ocvrs_create_string(ret.c_str());
	}

	// std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::get_1() generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_get_1_const(const std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* instance, cv::Ptr<cv::TrackerSamplerAlgorithm>** ocvrs_return) {
			cv::Ptr<cv::TrackerSamplerAlgorithm> ret = std::get<1>(*instance);
			*ocvrs_return = new cv::Ptr<cv::TrackerSamplerAlgorithm>(ret);
	}

	// std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::delete() generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLcv_String__cv_PtrLcv_TrackerSamplerAlgorithmGG_delete(std::pair<cv::String, cv::Ptr<cv::TrackerSamplerAlgorithm>>* instance) {
			delete instance;
	}

}


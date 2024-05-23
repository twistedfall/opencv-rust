extern "C" {
	// std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::new(InString, CppPassByVoidPtr) generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::new", vec![(pred!(const, ["arg", "arg_1"], ["cv::String", "cv::Ptr<cv::TrackerFeature>"]), _)]),
	std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_new_const_String_PtrLTrackerFeatureG(const char* arg, cv::Ptr<cv::TrackerFeature>* arg_1) {
			std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* ret = new std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>(cv::String(arg), *arg_1);
			return ret;
	}

	// std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::get_0() generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_get_0_const(const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* instance, void** ocvrs_return) {
			cv::String ret = std::get<0>(*instance);
			*ocvrs_return = ocvrs_create_string(ret.c_str());
	}

	// std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::get_1() generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_get_1_const(const std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* instance, cv::Ptr<cv::TrackerFeature>** ocvrs_return) {
			cv::Ptr<cv::TrackerFeature> ret = std::get<1>(*instance);
			*ocvrs_return = new cv::Ptr<cv::TrackerFeature>(ret);
	}

	// std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::delete() generated
	// ("std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLcv_String__cv_PtrLcv_TrackerFeatureGG_delete(std::pair<cv::String, cv::Ptr<cv::TrackerFeature>>* instance) {
			delete instance;
	}

}


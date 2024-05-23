extern "C" {
	// std::pair<cv::Ptr<cv::TrackerTargetState>, float>::new(CppPassByVoidPtr, Primitive) generated
	// ("std::pair<cv::Ptr<cv::TrackerTargetState>, float>::new", vec![(pred!(const, ["arg", "arg_1"], ["cv::Ptr<cv::TrackerTargetState>", "float"]), _)]),
	std::pair<cv::Ptr<cv::TrackerTargetState>, float>* std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_new_const_PtrLTrackerTargetStateG_float(cv::Ptr<cv::TrackerTargetState>* arg, float arg_1) {
			std::pair<cv::Ptr<cv::TrackerTargetState>, float>* ret = new std::pair<cv::Ptr<cv::TrackerTargetState>, float>(*arg, arg_1);
			return ret;
	}

	// std::pair<cv::Ptr<cv::TrackerTargetState>, float>::get_0() generated
	// ("std::pair<cv::Ptr<cv::TrackerTargetState>, float>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_get_0_const(const std::pair<cv::Ptr<cv::TrackerTargetState>, float>* instance, cv::Ptr<cv::TrackerTargetState>** ocvrs_return) {
			cv::Ptr<cv::TrackerTargetState> ret = std::get<0>(*instance);
			*ocvrs_return = new cv::Ptr<cv::TrackerTargetState>(ret);
	}

	// std::pair<cv::Ptr<cv::TrackerTargetState>, float>::get_1() generated
	// ("std::pair<cv::Ptr<cv::TrackerTargetState>, float>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_get_1_const(const std::pair<cv::Ptr<cv::TrackerTargetState>, float>* instance, float* ocvrs_return) {
			float ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<cv::Ptr<cv::TrackerTargetState>, float>::delete() generated
	// ("std::pair<cv::Ptr<cv::TrackerTargetState>, float>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLcv_PtrLcv_TrackerTargetStateG__floatG_delete(std::pair<cv::Ptr<cv::TrackerTargetState>, float>* instance) {
			delete instance;
	}

}


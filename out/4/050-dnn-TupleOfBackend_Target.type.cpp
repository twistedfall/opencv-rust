extern "C" {
	// std::pair<cv::dnn::Backend, cv::dnn::Target>::new(Enum, Enum) generated
	// ("std::pair<cv::dnn::Backend, cv::dnn::Target>::new", vec![(pred!(const, ["arg", "arg_1"], ["cv::dnn::Backend", "cv::dnn::Target"]), _)]),
	std::pair<cv::dnn::Backend, cv::dnn::Target>* std_pairLcv_dnn_Backend__cv_dnn_TargetG_new_const_Backend_Target(cv::dnn::Backend arg, cv::dnn::Target arg_1) {
			std::pair<cv::dnn::Backend, cv::dnn::Target>* ret = new std::pair<cv::dnn::Backend, cv::dnn::Target>(arg, arg_1);
			return ret;
	}

	// std::pair<cv::dnn::Backend, cv::dnn::Target>::get_0() generated
	// ("std::pair<cv::dnn::Backend, cv::dnn::Target>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_0_const(const std::pair<cv::dnn::Backend, cv::dnn::Target>* instance, cv::dnn::Backend* ocvrs_return) {
			cv::dnn::Backend ret = std::get<0>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<cv::dnn::Backend, cv::dnn::Target>::get_1() generated
	// ("std::pair<cv::dnn::Backend, cv::dnn::Target>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_1_const(const std::pair<cv::dnn::Backend, cv::dnn::Target>* instance, cv::dnn::Target* ocvrs_return) {
			cv::dnn::Target ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<cv::dnn::Backend, cv::dnn::Target>::delete() generated
	// ("std::pair<cv::dnn::Backend, cv::dnn::Target>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLcv_dnn_Backend__cv_dnn_TargetG_delete(std::pair<cv::dnn::Backend, cv::dnn::Target>* instance) {
			delete instance;
	}

}


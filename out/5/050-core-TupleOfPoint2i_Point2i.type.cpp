extern "C" {
	// std::pair<cv::Point2i, cv::Point2i>::new(SimpleClass, SimpleClass) generated
	// ("std::pair<cv::Point2i, cv::Point2i>::new", vec![(pred!(const, ["arg", "arg_1"], ["cv::Point2i", "cv::Point2i"]), _)]),
	std::pair<cv::Point2i, cv::Point2i>* std_pairLcv_Point2i__cv_Point2iG_new_const_Point2i_Point2i(cv::Point2i* arg, cv::Point2i* arg_1) {
			std::pair<cv::Point2i, cv::Point2i>* ret = new std::pair<cv::Point2i, cv::Point2i>(*arg, *arg_1);
			return ret;
	}

	// std::pair<cv::Point2i, cv::Point2i>::get_0() generated
	// ("std::pair<cv::Point2i, cv::Point2i>::get_0", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_Point2i__cv_Point2iG_get_0_const(const std::pair<cv::Point2i, cv::Point2i>* instance, cv::Point2i* ocvrs_return) {
			cv::Point2i ret = std::get<0>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<cv::Point2i, cv::Point2i>::get_1() generated
	// ("std::pair<cv::Point2i, cv::Point2i>::get_1", vec![(pred!(const, [], []), _)]),
	void std_pairLcv_Point2i__cv_Point2iG_get_1_const(const std::pair<cv::Point2i, cv::Point2i>* instance, cv::Point2i* ocvrs_return) {
			cv::Point2i ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}

	// std::pair<cv::Point2i, cv::Point2i>::delete() generated
	// ("std::pair<cv::Point2i, cv::Point2i>::delete", vec![(pred!(mut, [], []), _)]),
	void std_pairLcv_Point2i__cv_Point2iG_delete(std::pair<cv::Point2i, cv::Point2i>* instance) {
			delete instance;
	}

}


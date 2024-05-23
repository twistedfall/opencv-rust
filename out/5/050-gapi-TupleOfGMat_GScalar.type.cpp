extern "C" {
	// std::tuple<cv::GMat, cv::GScalar>::new(TraitClass, TraitClass) generated
	// ("std::tuple<cv::GMat, cv::GScalar>::new", vec![(pred!(const, ["arg", "arg_1"], ["cv::GMat", "cv::GScalar"]), _)]),
	std::tuple<cv::GMat, cv::GScalar>* std_tupleLcv_GMat__cv_GScalarG_new_const_GMat_GScalar(cv::GMat* arg, cv::GScalar* arg_1) {
			std::tuple<cv::GMat, cv::GScalar>* ret = new std::tuple<cv::GMat, cv::GScalar>(*arg, *arg_1);
			return ret;
	}

	// std::tuple<cv::GMat, cv::GScalar>::get_0() generated
	// ("std::tuple<cv::GMat, cv::GScalar>::get_0", vec![(pred!(const, [], []), _)]),
	void std_tupleLcv_GMat__cv_GScalarG_get_0_const(const std::tuple<cv::GMat, cv::GScalar>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<0>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}

	// std::tuple<cv::GMat, cv::GScalar>::get_1() generated
	// ("std::tuple<cv::GMat, cv::GScalar>::get_1", vec![(pred!(const, [], []), _)]),
	void std_tupleLcv_GMat__cv_GScalarG_get_1_const(const std::tuple<cv::GMat, cv::GScalar>* instance, cv::GScalar** ocvrs_return) {
			cv::GScalar ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GScalar(ret);
	}

	// std::tuple<cv::GMat, cv::GScalar>::delete() generated
	// ("std::tuple<cv::GMat, cv::GScalar>::delete", vec![(pred!(mut, [], []), _)]),
	void std_tupleLcv_GMat__cv_GScalarG_delete(std::tuple<cv::GMat, cv::GScalar>* instance) {
			delete instance;
	}

}

